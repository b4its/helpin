import { defineEventHandler, readBody, createError } from 'h3'
import { writeFile, mkdir } from 'node:fs/promises'
import { existsSync } from 'node:fs'
import { join } from 'node:path'
import { randomUUID } from 'node:crypto'

/**
 * Upload gambar produk → disimpan ke folder public/media milik Nuxt.
 * Body: { filename, data (data URL base64) } → return { url: "/media/xxx.ext" }
 */
export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  if (!body?.data) {
    throw createError({ statusCode: 400, statusMessage: 'Data gambar tidak ada' })
  }

  const match = /^data:(.+?);base64,(.*)$/.exec(body.data)
  if (!match) {
    throw createError({ statusCode: 400, statusMessage: 'Format data URL tidak valid' })
  }

  const mime = match[1]
  const b64 = match[2]
  const ext = (mime.split('/')[1] || 'png').split('+')[0]
  const safeBase = String(body.filename || 'img')
    .replace(/\.[^.]+$/, '')
    .replace(/[^a-zA-Z0-9._-]/g, '_')
    .slice(0, 40)
  const name = `${Date.now()}-${randomUUID().slice(0, 8)}-${safeBase}.${ext}`

  const cwd = process.cwd()
  // Produksi (node .output/server): public ada di .output/public; dev: ./public
  const baseDir = existsSync(join(cwd, '.output/public'))
    ? join(cwd, '.output/public')
    : join(cwd, 'public')
  const mediaDir = join(baseDir, 'media')

  await mkdir(mediaDir, { recursive: true })
  await writeFile(join(mediaDir, name), Buffer.from(b64, 'base64'))

  return { url: `/media/${name}` }
})
