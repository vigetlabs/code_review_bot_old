# Code Review Bot

A bot that posts your PRs for code review in slack and automatically updates the message with the pr status (reviewed, approved, closed, or merged).

## Repo Setup

There are 2 requirements to connect a repository to the bot:

 * A GitHub webhook configured on the repo pointing to the bot's endpoint
 * Read access to the repo for the [viget](https://github.com/orgs/vigetlabs/people/viget) GH account

### GitHub Webhook

 1. Navigate to the repo *Settings* page
 2. Navigate to the *Webhooks* tab
 3. Click *Add webhook*. You may be prompted to confirm your password.
 4. For *Payload URL*, enter `https://tbfpsdbx2g.execute-api.us-east-1.amazonaws.com/payload`
 5. For *Content type*, select `application/json`
 6. For *Which events would you like to trigger this webhook?*, select `Let me select individual events.`
 7. Under `Let me select individual events.`, toggle `Pushes` to _off_, then toggle `Pull request reviews` and `Pull requests` to _on_
 8. At the bottom, ensure `Active` is toggled _on_
 9. Click *Add webhook*

#### Screencap

![crb-webhook-screencap.gif](crb-webhook-screencap.gif)

### `viget` Access

 1. Navigate to the organization that owns the repo
 2. Navigate to the *People* page
 3. Add the [viget](https://github.com/orgs/vigetlabs/people/viget) account as a _Member_ or _Outside collaborator_
 4. Ensure the account has access to the repo

## Documentation

TODO
