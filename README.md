# Jira CLI Tool

This is a tool for interacting with jira sprints via a CLI. It is basically a wrapper around the jira api. Under active development, please post a github issue if something is giving you trouble. I'd say it's not even ready for alpha testing yet since all it can do is store an API token in a local file. What would be super helpful is if you want a feature that is not in this list, then please make a PR adding it to this read me! 

This is a funzies project - not something anyone is being paid to do in any real way. We will try to maintain backwards compatibility but we aren't making any guarantees.

## Why?

Jira has a UI, technically. It is buggy, slow, loads weirdly. No shade to their engineers, building stuff is hard, but I feel like they haven't reached their final form. I looked around for other tools and found nothing lightweight enough for my preferences. Ultimately I chose to build this during Rust so I can learn something new and and learn a little more about "systems programming".

My goal is for this tool to feel simple to use - as reliable and effective as a hammer (or grep): something you can script against, something that is so straightforward you don't need to read pages of documentations just to use it, something that it is so useful you don't think of it (or if you do you thank it's creator). I also want to see what quality of software I can build when I am not facing the constraints of working in the office (under time, team, and monetary pressure). Hopefully this feels like a return to the simple building blocks everyone falls in love with when they learn to code.

## Links

- jira's cli tool, this feels more admin oriented. try it [here](https://marketplace.atlassian.com/apps/6398/jira-command-line-interface-cli?tab=overview&hosting=cloud)
- someone made a jira cli tool! it looks cool but a little overly complex, try it [here] (https://github.com/ankitpokhrel/jira-cli)
- need to create a jira api token? try [here](https://id.atlassian.com/manage-profile/security/api-tokens)

## Feature Wish List

v0.0 - a bare bones tool for interacting with tickets throughout a sprint
- [x] store a local copy of your jira api token
- [ ] display ticket data given it's ID, like "EN-1"
- [ ] modify a ticket's status one state to another ("In Progress", "Done") - it would be cool if this is based on
- [ ] modify a ticket's metadata (description, story points, epic, etc)
- [ ] view all tickets in the current sprint (summary and status)
- [ ] create a new ticket (give it a title, summary, pick a sprint to add it to, an epic, and a story point estimation)

v0.1 - a tool you can use for your sprint planning work
- [ ] view tickets in active sprint grouped by team members
- [ ] look at tickets by sprint and in backlog (ideally filterable by whether they are assigned to you and you creaeted them
- [ ] make which metadata you care about configurable
- [ ] bulk edit tickets

v1.0 - a tool so good you don't even need to use the jira web ui
- [ ] comment on tickets
- [ ] "@" other jira users
- [ ] some way to view and use notifications (I don't really use these but if you do chime in)
