FROM alpine:latest

COPY bin/project-issue-state-sync /project-issue-state-sync

ENTRYPOINT ["/project-issue-state-sync"]