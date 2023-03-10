FROM node:19-alpine3.16 
WORKDIR /frontend

COPY ["package.json", "package-lock.json*", "./"]

RUN yarn

COPY . .

RUN yarn build --mode docker

CMD ["node", "build/index.js"]