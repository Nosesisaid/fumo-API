FROM node:18

WORKDIR /app

COPY package.json ./
RUN yarn install

COPY . .
RUN yarn build
EXPOSE 80
CMD ["node", "dist/server.js"]