import antfu from '@antfu/eslint-config'

export default antfu(
  {
    ignores: [
      './src-tauri',
      './pkg_service',
      './public',
      './rfcs',
    ],
    unocss: true,
    formatters: true,
    toml: false,
  },
)
