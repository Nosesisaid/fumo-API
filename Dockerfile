LABEL org.opencontainers.image.source https://github.com/Nosesisaid/Fumo-API
FROM node:18

WORKDIR /app

COPY package.json ./
RUN yarn install

COPY . .
RUN yarn build
EXPOSE 8080
CMD ["node", "dist/server.js"]