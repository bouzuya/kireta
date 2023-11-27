FROM node:20.9.0-bookworm
RUN usermod --home /home/bouzuya --login bouzuya node && \
  mkdir -p /home/bouzuya/kireta
WORKDIR /home/bouzuya/kireta
RUN npm install --global eas-cli fixpack
