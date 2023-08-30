import { ExpoConfig, ConfigContext } from "expo/config";

export default ({ config }: ConfigContext): ExpoConfig => {
  return {
    ...config,
    extra: {
      backendBaseUrl: process.env.BACKEND_BASE_URL,
    },
    name: "kireta",
    slug: "kireta",
  };
};
