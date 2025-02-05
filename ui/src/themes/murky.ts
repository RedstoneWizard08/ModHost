import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const murkyTheme: CustomThemeConfig = {
    name: "murky",

    properties: {
        // =~= Theme Properties =~=
        "--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
        "--theme-font-color-base": "0 0 0",
        "--theme-font-color-dark": "255 255 255",
        "--theme-rounded-base": "9999px",
        "--theme-rounded-container": "12px",
        "--theme-border-base": "2px",
        // =~= Theme On-X Colors =~=
        "--on-primary": "0 0 0",
        "--on-secondary": "0 0 0",
        "--on-tertiary": "0 0 0",
        "--on-success": "0 0 0",
        "--on-warning": "0 0 0",
        "--on-error": "255 255 255",
        "--on-surface": "255 255 255",
        // =~= Theme Colors  =~=
        // primary | #ec3936
        "--color-primary-50": "252 225 225", // #fce1e1
        "--color-primary-100": "251 215 215", // #fbd7d7
        "--color-primary-200": "250 206 205", // #facecd
        "--color-primary-300": "247 176 175", // #f7b0af
        "--color-primary-400": "242 116 114", // #f27472
        "--color-primary-500": "236 57 54", // #ec3936
        "--color-primary-600": "212 51 49", // #d43331
        "--color-primary-700": "177 43 41", // #b12b29
        "--color-primary-800": "142 34 32", // #8e2220
        "--color-primary-900": "116 28 26", // #741c1a
        // secondary | #df7630
        "--color-secondary-50": "250 234 224", // #faeae0
        "--color-secondary-100": "249 228 214", // #f9e4d6
        "--color-secondary-200": "247 221 203", // #f7ddcb
        "--color-secondary-300": "242 200 172", // #f2c8ac
        "--color-secondary-400": "233 159 110", // #e99f6e
        "--color-secondary-500": "223 118 48", // #df7630
        "--color-secondary-600": "201 106 43", // #c96a2b
        "--color-secondary-700": "167 89 36", // #a75924
        "--color-secondary-800": "134 71 29", // #86471d
        "--color-secondary-900": "109 58 24", // #6d3a18
        // tertiary | #28a99c
        "--color-tertiary-50": "223 242 240", // #dff2f0
        "--color-tertiary-100": "212 238 235", // #d4eeeb
        "--color-tertiary-200": "201 234 230", // #c9eae6
        "--color-tertiary-300": "169 221 215", // #a9ddd7
        "--color-tertiary-400": "105 195 186", // #69c3ba
        "--color-tertiary-500": "40 169 156", // #28a99c
        "--color-tertiary-600": "36 152 140", // #24988c
        "--color-tertiary-700": "30 127 117", // #1e7f75
        "--color-tertiary-800": "24 101 94", // #18655e
        "--color-tertiary-900": "20 83 76", // #14534c
        // success | #85c328
        "--color-success-50": "237 246 223", // #edf6df
        "--color-success-100": "231 243 212", // #e7f3d4
        "--color-success-200": "225 240 201", // #e1f0c9
        "--color-success-300": "206 231 169", // #cee7a9
        "--color-success-400": "170 213 105", // #aad569
        "--color-success-500": "133 195 40", // #85c328
        "--color-success-600": "120 176 36", // #78b024
        "--color-success-700": "100 146 30", // #64921e
        "--color-success-800": "80 117 24", // #507518
        "--color-success-900": "65 96 20", // #416014
        // warning | #cdaf18
        "--color-warning-50": "248 243 220", // #f8f3dc
        "--color-warning-100": "245 239 209", // #f5efd1
        "--color-warning-200": "243 235 197", // #f3ebc5
        "--color-warning-300": "235 223 163", // #ebdfa3
        "--color-warning-400": "220 199 93", // #dcc75d
        "--color-warning-500": "205 175 24", // #cdaf18
        "--color-warning-600": "185 158 22", // #b99e16
        "--color-warning-700": "154 131 18", // #9a8312
        "--color-warning-800": "123 105 14", // #7b690e
        "--color-warning-900": "100 86 12", // #64560c
        // error | #ac394b
        "--color-error-50": "243 225 228", // #f3e1e4
        "--color-error-100": "238 215 219", // #eed7db
        "--color-error-200": "234 206 210", // #eaced2
        "--color-error-300": "222 176 183", // #deb0b7
        "--color-error-400": "197 116 129", // #c57481
        "--color-error-500": "172 57 75", // #ac394b
        "--color-error-600": "155 51 68", // #9b3344
        "--color-error-700": "129 43 56", // #812b38
        "--color-error-800": "103 34 45", // #67222d
        "--color-error-900": "84 28 37", // #541c25
        // surface | #1d4858
        "--color-surface-50": "221 228 230", // #dde4e6
        "--color-surface-100": "210 218 222", // #d2dade
        "--color-surface-200": "199 209 213", // #c7d1d5
        "--color-surface-300": "165 182 188", // #a5b6bc
        "--color-surface-400": "97 127 138", // #617f8a
        "--color-surface-500": "29 72 88", // #1d4858
        "--color-surface-600": "26 65 79", // #1a414f
        "--color-surface-700": "22 54 66", // #163642
        "--color-surface-800": "17 43 53", // #112b35
        "--color-surface-900": "14 35 43", // #0e232b
    },
};
