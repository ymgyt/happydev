module.exports = {
    "env": {
        "browser": true,
        "es6": true
    },
    "extends": [
        "plugin:react/recommended",
        "airbnb",
        "airbnb/hooks",
        "plugin:import/errors",
        "plugin:import/warnings",
        "plugin:import/typescript",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
        "plugin:prettier/recommended",
        "prettier",
        "prettier/@typescript-eslint",
        "prettier/react",
        "prettier/standard"
    ],
    "globals": {
        "Atomics": "readonly",
        "SharedArrayBuffer": "readonly"
    },
    "parser": "@typescript-eslint/parser",
    "parserOptions": {
        "ecmaFeatures": {
            "jsx": true
        },
        "ecmaVersion": 2020,
        "project": "./tsconfig.eslint.json",
        "sourceType": "module",
        "tsconfigRootDir": __dirname,
    },
    "plugins": [
        "@typescript-eslint",
        "import",
        "jsx-a11y",
        "prettier",
        "react",
        "react-hooks",
    ],
    "rules": {
        "react/jsx-props-no-spreading": "off", // <A ...value /> のようなpropsの渡し方を禁止する
        "import/extensions": [
            "error",
            "ignorePackages",
            {
                "js": "never",
                "jsx": "never",
                "ts": "never",
                "tsx": "never",
            }
        ],
        "no-use-before-define": "off", // versionがあってないbugらしい
        "react/jsx-filename-extension": [
            "error",
            {
                extensions: [".jsx", ".tsx"]
            }
        ],
    },
    "overrides": [
        {
            "files": ["*.tsx"],
            "rules": {
                "react/prop-types": "off"
            }
        }
    ],
  "settings": {
      "import/resolver": {
          node: {
              paths: ["src"],
          },
      }
  }
};
