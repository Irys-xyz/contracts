FROM node:16 

WORKDIR ./validators
COPY ./validators/ts/contract.ts ./ts/contract.ts
COPY ./validators/package.json  ./package.json
RUN yarn 

WORKDIR ./gateway

COPY ./gateway/tsconfig.json ./tsconfig.json
COPY ./gateway/src ./src
COPY ./gateway/package.json ./package.json
COPY ./gateway/yarn.lock ./yarn.lock


RUN yarn
RUN yarn build

EXPOSE 3000

CMD ["node","./gateway/dist/gateway/src/index.js"]
