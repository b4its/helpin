import { defineEventHandler, getRouterParam, setHeader, createError } from 'h3'
import { readFile } from 'node:fs/promises'
import { existsSync } from 'node:fs'
import { join, extname } from 'node:path'

const MIME: Record<string, string> = {
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.webp': 'image/webp',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.bmp': 'image/bmp',
  '.avif': 'image/avif',
}

/**
 * Sajikan file gambar dari folder public/media.
 * File yang diunggah saat runtime tidak ada di manifest aset statis Nitro,
 * sehingga perlu route eksplisit untuk membacanya langsung dari disk.
 */
export default defineEventHandler(async (event) => {
  const raw = getRouterParam(event, 'path') || ''
  // Cegah path traversal
  const safe = decodeURIComponent(raw).split('..').join('').replace(/^\/+/, '')

  const cwd = process.cwd()
  const baseDir = existsSync(join(cwd, '.output/public'))
    ? join(cwd, '.output/public')
    : join(cwd, 'public')
  const file = join(baseDir, 'media', safe)

  if (!file.startsWith(join(baseDir, 'media')) || !existsSync(file)) {
    throw createError({ statusCode: 404, statusMessage: 'Media tidak ditemukan' })
  }

  const buf = await readFile(file)
  const ext = extname(file).toLowerCase()
  setHeader(event, 'Content-Type', MIME[ext] || 'application/octet-stream')
  setHeader(event, 'Cache-Control', 'public, max-age=86400')
  return buf
})
