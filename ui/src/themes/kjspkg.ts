import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const kjspkgTheme: CustomThemeConfig = {
    name: "kjspkg",

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
        "--on-secondary": "255 255 255",
        "--on-tertiary": "255 255 255",
        "--on-success": "0 0 0",
        "--on-warning": "0 0 0",
        "--on-error": "255 255 255",
        "--on-surface": "255 255 255",
        // =~= Theme Colors  =~=
        // primary | #bd75e1
        "--color-primary-50": "245 234 251", // #f5eafb
        "--color-primary-100": "242 227 249", // #f2e3f9
        "--color-primary-200": "239 221 248", // #efddf8
        "--color-primary-300": "229 200 243", // #e5c8f3
        "--color-primary-400": "209 158 234", // #d19eea
        "--color-primary-500": "189 117 225", // #bd75e1
        "--color-primary-600": "170 105 203", // #aa69cb
        "--color-primary-700": "142 88 169", // #8e58a9
        "--color-primary-800": "113 70 135", // #714687
        "--color-primary-900": "93 57 110", // #5d396e
        // secondary | #624270
        "--color-secondary-50": "231 227 234", // #e7e3ea
        "--color-secondary-100": "224 217 226", // #e0d9e2
        "--color-secondary-200": "216 208 219", // #d8d0db
        "--color-secondary-300": "192 179 198", // #c0b3c6
        "--color-secondary-400": "145 123 155", // #917b9b
        "--color-secondary-500": "98 66 112", // #624270
        "--color-secondary-600": "88 59 101", // #583b65
        "--color-secondary-700": "74 50 84", // #4a3254
        "--color-secondary-800": "59 40 67", // #3b2843
        "--color-secondary-900": "48 32 55", // #302037
        // tertiary | #663a83
        "--color-tertiary-50": "232 225 236", // #e8e1ec
        "--color-tertiary-100": "224 216 230", // #e0d8e6
        "--color-tertiary-200": "217 206 224", // #d9cee0
        "--color-tertiary-300": "194 176 205", // #c2b0cd
        "--color-tertiary-400": "148 117 168", // #9475a8
        "--color-tertiary-500": "102 58 131", // #663a83
        "--color-tertiary-600": "92 52 118", // #5c3476
        "--color-tertiary-700": "77 44 98", // #4d2c62
        "--color-tertiary-800": "61 35 79", // #3d234f
        "--color-tertiary-900": "50 28 64", // #321c40
        // success | #4ddb83
        "--color-success-50": "228 250 236", // #e4faec
        "--color-success-100": "219 248 230", // #dbf8e6
        "--color-success-200": "211 246 224", // #d3f6e0
        "--color-success-300": "184 241 205", // #b8f1cd
        "--color-success-400": "130 230 168", // #82e6a8
        "--color-success-500": "77 219 131", // #4ddb83
        "--color-success-600": "69 197 118", // #45c576
        "--color-success-700": "58 164 98", // #3aa462
        "--color-success-800": "46 131 79", // #2e834f
        "--color-success-900": "38 107 64", // #266b40
        // warning | #c8811e
        "--color-warning-50": "247 236 221", // #f7ecdd
        "--color-warning-100": "244 230 210", // #f4e6d2
        "--color-warning-200": "241 224 199", // #f1e0c7
        "--color-warning-300": "233 205 165", // #e9cda5
        "--color-warning-400": "217 167 98", // #d9a762
        "--color-warning-500": "200 129 30", // #c8811e
        "--color-warning-600": "180 116 27", // #b4741b
        "--color-warning-700": "150 97 23", // #966117
        "--color-warning-800": "120 77 18", // #784d12
        "--color-warning-900": "98 63 15", // #623f0f
        // error | #e21854
        "--color-error-50": "251 220 229", // #fbdce5
        "--color-error-100": "249 209 221", // #f9d1dd
        "--color-error-200": "248 197 212", // #f8c5d4
        "--color-error-300": "243 163 187", // #f3a3bb
        "--color-error-400": "235 93 135", // #eb5d87
        "--color-error-500": "226 24 84", // #e21854
        "--color-error-600": "203 22 76", // #cb164c
        "--color-error-700": "170 18 63", // #aa123f
        "--color-error-800": "136 14 50", // #880e32
        "--color-error-900": "111 12 41", // #6f0c29
        // surface | #420d63
        "--color-surface-50": "227 219 232", // #e3dbe8
        "--color-surface-100": "217 207 224", // #d9cfe0
        "--color-surface-200": "208 195 216", // #d0c3d8
        "--color-surface-300": "179 158 193", // #b39ec1
        "--color-surface-400": "123 86 146", // #7b5692
        "--color-surface-500": "66 13 99", // #420d63
        "--color-surface-600": "59 12 89", // #3b0c59
        "--color-surface-700": "50 10 74", // #320a4a
        "--color-surface-800": "40 8 59", // #28083b
        "--color-surface-900": "32 6 49", // #200631
    },
};
