[![CircleCI](https://circleci.com/gh/vigetlabs/code_review_bot.svg?style=svg&circle-token=35183e609cd216df24f1668009b60bbcdfe13c73)](https://circleci.com/gh/vigetlabs/code_review_bot)

# Code Review Bot Test
A bot that posts your PRs for code review in slack and automatically updates the message with the pr status (reviewed, approved, closed, or merged).

## Adding A Webhook (recommended)

1. On your github repo page click `Settings` then `Webhooks`
2. Click `Add webhook`
3. Add http://crbot.vigetx.com/github_event to the `Payload URL` field
4. Set `Content type` to `application/json`
5. Select `Let me select individual events` radio button
6. Check the `Pull requests` and `Pull request reviews` boxes
7. Click `Add webhook` button

8. (Temporary) Make sure joeyjoejoejr has read access to the repo (for file type
   detection)

## Running locally (using docker)
* Install [Docker for mac](https://docs.docker.com/v17.12/docker-for-mac/install/)
* Copy and complete `.env.example` to `.env` (Credentials are in 1password under Code Review Bot Credentials)
* Run `docker-compose build` (takes about 5 minutes)
* Run `docker-compose run web diesel database setup` to create and migrate your
  database
* Run `docker-compose up`
* You will probably want to use ngrok and set up a slack slash command to test
  with (more on that later, ask Joe if you need help with this)

* For migrations `docker-compose run web diesel migration run`

## Deploying
### Prerequisites
* Proper keys in `.env`
* SSH access to crbot.vigetx.com
* The server set up as a docker-machine env `docker-machine create --driver generic --generic-ip-address <ipofdockerhost> --generic-ssh-user root`

### To deploy
* Run `eval $(docker-machine env crbot-prod)` to setup your environment
* Run
```
$ docker-compose -f docker-compose.yml -f docker-compose.prod.yml build web
$ docker-compose -f docker-compose.yml -f docker-compose.prod.yml up --no-deps -d web
```
* For migrations `docker-compose -f docker-compose.yml -f docker-compose.prod.yml run web diesel migration run`
