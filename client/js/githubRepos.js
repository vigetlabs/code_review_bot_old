import { debounce } from './utils'

let repoListElements
let repoListEl

async function* getRepoList() {
    let repoList = []
    let nextUrl = "/github/repos"

    while (nextUrl != null) {
        let res = await fetch(nextUrl, { credentials: 'same-origin' })
        let resJson = await res.json()

        if (resJson.pagination.next) {
            nextUrl = `/github/repos${new URL(resJson.pagination.next).search}`
        } else {
            nextUrl = null
        }

        yield resJson.repos
    }
}

async function postForm(formEl) {
    let res = await fetch(formEl.action, {
        body: urlencodeFormData(new FormData(formEl)),
        method: 'post',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        credentials: 'same-origin',
    })
    return await res.json()
}

async function handleCreateSubmit(e, repo) {
    const formEl = e.target
    e.preventDefault()

    try {
        const hook = await postForm(formEl)
        formEl.replaceWith(removeWebhookForm(repo, hook))
    } catch (e) {
        console.error(e)
    }
}

async function handleRemoveSubmit(e, repo) {
    const formEl = e.target
    e.preventDefault()

    try {
        formEl.replaceWith(createWebhookForm(repo))
    } catch (e) {
        console.error(e)
    }
}

function handleSearch(e) {
    let searchVal = e.target.value

    const filtered = repoListElements.filter(
        el => el.textContent.toLowerCase().includes(searchVal.toLowerCase())
    )

    repoListEl.innerHTML = ''
    repoListEl.append(...filtered)
}

function urlencodeFormData(fd) {
    var params = new URLSearchParams()
    for (var pair of fd.entries()) {
        typeof pair[1] == 'string' && params.append(pair[0], pair[1])
    }
    return params.toString()
}

function repoListElement(repo) {
    const li = document.createElement('li')
    const div = document.createElement('div')
    div.className = "uk-flex uk-flex-between"
    const text = document.createTextNode(repo.repo.full_name)
    div.appendChild(text)

    if (repo.repo.permissions.admin) {
        const webhookForm = repo.webhook == null
            ? createWebhookForm(repo.repo)
            : removeWebhookForm(repo.repo, repo.webhook)

        div.appendChild(webhookForm)
    }

    li.appendChild(div)

    return li
}

function createWebhookForm(repo) {
    const formEl = document.createElement('form')
    formEl.action = '/github/webhooks'
    formEl.method = 'POST'

    const ownerInput = document.createElement('input')
    ownerInput.type = 'hidden'
    ownerInput.value = repo.owner.login
    ownerInput.name = 'owner'
    formEl.appendChild(ownerInput)

    const nameInput = document.createElement('input')
    nameInput.type = 'hidden'
    nameInput.value = repo.name
    nameInput.name = 'name'
    formEl.appendChild(nameInput)

    const button = document.createElement('button')
    button.innerHTML = 'Add Webhook'
    button.className = 'uk-button uk-button-primary uk-button-small'
    formEl.appendChild(button)

    formEl.addEventListener('submit', (e) => handleCreateSubmit(e, repo))

    return formEl
}

function removeWebhookForm(repo, hook) {
    const formEl = document.createElement('form')
    formEl.action = `/github/webhooks/${hook.id}`
    formEl.method = 'POST'

    const methodInput = document.createElement('input')
    methodInput.type = 'hidden'
    methodInput.value = 'delete'
    methodInput.name = '_method'
    formEl.appendChild(methodInput)

    const button = document.createElement('button')
    button.innerHTML = 'Remove Webhook'
    button.className = 'uk-button uk-button-danger uk-button-small'
    formEl.appendChild(button)

    formEl.addEventListener('submit', (e) => handleRemoveSubmit(e, repo))

    return formEl
}

export const init = async () => {
    repoListEl = document.getElementById('github-repos')
    const spinner = repoListEl.firstElementChild
    const searchInput = document.getElementById('repo-search')

    if (repoListEl != null) {
        try {
            for await (let repos of getRepoList()) {
                const repoElements = repos.map(repoListElement)

                repoListEl.append(...repoElements)
            }

            spinner.remove()

            repoListElements = [...repoListEl.children]

            searchInput.removeAttribute('disabled')
            searchInput.addEventListener('keydown', debounce(handleSearch, 1000, false))
        } catch (err) {
            console.error(err)
            const error = document.createElement('li')
            error.innerHTML = 'Something went wrong'

            spinner.replaceWith(error)
        }
    }
}
