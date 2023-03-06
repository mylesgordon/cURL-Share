FROM node:19-alpine3.16 
WORKDIR /app

COPY ["package.json", "package-lock.json*", "./"]

RUN yarn

COPY . .

RUN yarn build

CMD [ "node", "build/index.js" ]