# Lightweight runtime-only Dockerfile
# Pre-build locally with: pnpm run build
# Then copy the .output folder into the image

FROM node:22-alpine

WORKDIR /app

# Copy pre-built output (run `pnpm run build` locally first)
COPY .output .output

ENV HOST=0.0.0.0
ENV PORT=3000
ENV NITRO_PORT=3000

EXPOSE 3000

CMD ["node", ".output/server/index.mjs"]
