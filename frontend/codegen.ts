import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "http://localhost:8080/graphql",
  documents: ["lib/graphql/operations/**/*.graphql"],
  generates: {
    "./lib/graphql/generated/": {
      preset: "client",
      plugins: [],
      presetConfig: {
        gqlTagName: "gql",
        fragmentMasking: false,
      },
      config: {
        scalars: {
          UUID: "string",
        },
      },
    },
    "./lib/graphql/generated/urql.ts": {
      plugins: ["typescript", "typescript-operations", "typescript-urql"],
      config: {
        withHooks: true,
        urqlImportFrom: "@urql/next",
        scalars: {
          UUID: "string",
        },
      },
    },
  },
  ignoreNoDocuments: true,
};

export default config;
