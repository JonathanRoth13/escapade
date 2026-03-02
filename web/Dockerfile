FROM node:22-alpine AS base


# ---------- Dependencies ----------
FROM base AS deps

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci


# ---------- Build ----------
FROM base AS build

WORKDIR /app

COPY --from=deps /app/node_modules ./node_modules
COPY . .

RUN npm run build


# ---------- Runtime ----------
FROM base

WORKDIR /app

ENV NODE_ENV=production
ENV ENGINE_URL=http://engine:8080
ENV HOSTNAME=0.0.0.0

COPY --from=build /app/.next/standalone ./
COPY --from=build /app/.next/static ./.next/static
COPY --from=build /app/public ./public

EXPOSE 3000

CMD ["node", "server.js"]
