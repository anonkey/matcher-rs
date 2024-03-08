FROM rust:alpine3.19

COPY . /app

WORKDIR /app


RUN SKIP_GIT_HOOKS=true ./scripts/init-dev-env.sh

CMD ['/bin/bash', '-c', './scripts/bump-versions.sh && if [[ -z `git status --porcelain` ]]; then echo "true"; fi']
