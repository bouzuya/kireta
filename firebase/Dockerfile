FROM node:18-alpine
RUN apk --no-cache add openjdk11-jre-headless \
  && java --version \
  && npm install --global firebase-tools@12.7.0 \
  && firebase --version
WORKDIR /firebase
CMD ["firebase", "emulators:start"]
