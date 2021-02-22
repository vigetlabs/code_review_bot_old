import UIKit from 'uikit'

export const init = () => {
    const formEl = document.getElementById('setup-form')
    const switcher = document.querySelector('[uk-switcher]')

    if (formEl != null) {
        const buttons = document.querySelectorAll('a.uk-button')
        const inputs = formEl.querySelectorAll('input')

        for (const input of inputs) {
            input.addEventListener('blur', e => input.classList.add('focused'))
        }

        for (const button of buttons) {
            button.addEventListener('click', e => {
                e.preventDefault()
                let index = parseInt(button.getAttribute('next-index'), 10)
                let inputs = button.parentElement.parentElement.querySelectorAll('input')
                let valid = true

                for (const input of [...inputs].reverse()) {
                    valid &= input.reportValidity()
                }

                if (valid) {
                    UIKit.switcher(switcher).show(index)
                }
            })
        }
    }
}
