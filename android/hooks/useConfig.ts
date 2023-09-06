import Constants from "expo-constants";

type Config = {
  backendBaseUrl: string;
};

export function useConfig(): Config {
  const extra: Record<string, unknown> = Constants.expoConfig?.extra ?? {};
  const backendBaseUrl = extra.backendBaseUrl ?? null;
  if (backendBaseUrl === null || typeof backendBaseUrl !== "string")
    throw new Error("BACKEND_BASE_URL is not defined or not a string");
  return { backendBaseUrl };
}
