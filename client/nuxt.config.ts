// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxtjs/tailwindcss',
    'nuxt-headlessui',
  ],
  app: {
    head: {
      // link: [{
      //   rel: 'preload',
      //   as: 'font',
      //   type: 'font/otf',
      //   href: '/Font.otf',
      //   crossorigin: '',
      // }]
    },
  },
  css: [
    '~/assets/style.scss',
  ],
  devServer: {
    port: 80,
  },
  routeRules: {
    '/json': {
      proxy: 'http://192.168.1.77:7878/json',
    },
    '/api/**': {
      proxy: 'http://192.168.1.77:7878/**',
    },
  },
  devtools: { enabled: false }
});