FROM node:alpine AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm build

FROM base

# Install curl for healthcheck
RUN apk add --no-cache curl

COPY --from=prod-deps /app/node_modules /app/node_modules
COPY --from=build /app/build /app/build
EXPOSE 3000
CMD ["pnpm", "start"]