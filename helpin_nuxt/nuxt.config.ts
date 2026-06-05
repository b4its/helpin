export default defineNuxtConfig({
  srcDir: 'app',
  dir: {
    pages: 'pages',
  },
  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxt/icon',
    '@vueuse/motion/nuxt'
  ],
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  app: {
    head: {
      title: 'Helpin',
    }
  },

  // 2. ATUR HOST KE '0.0.0.0' DISINI SETELAH SRCDIR DITEGASKAN
  devServer: {
    host: '0.0.0.0',
    port: 3000
  },
  tailwindcss: {
    config: {
      content: [
        "./app/**/*.{js,vue,ts}",
        "./app/**/*.vue"
      ],
      theme: {
        extend: {
          colors: {
            kopDark: '#1B4332',   // Hijau tua untuk tombol dan teks utama
            kopPrimary: '#2D6A4F', // Hijau utama
            kopLight: '#D8F3DC',  // Hijau sangat muda untuk aksen background
            kopSurface: '#F8FBF8' // Putih kehijauan untuk latar
          },
          fontFamily: {
            sans: ['Inter', 'sans-serif'],
          },
          boxShadow: {
            soft: '0 20px 40px -15px rgba(45, 106, 79, 0.1)',
            glow: '0 0 40px 10px rgba(216, 243, 220, 0.6)'
          }
        }
      }
    }
  }
})