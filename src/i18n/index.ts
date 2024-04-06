import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh_CN.ts'
import enUS from './locales/en_US.ts'

const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: 'zh-cn', // 默认显示语言
  messages: {
    'zh-cn': zhCN,
    'en-us': enUS
  }
})

export default i18n