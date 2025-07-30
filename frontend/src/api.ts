let baseUrl = import.meta.env.VITE_API_BASE || "http://localhost:3001";

export function getBaseUrl() {
  return baseUrl;
}

export function setBaseUrl(url: string) {
  baseUrl = url;
}

export async function apiGet<T>(path: string): Promise<T> {
  const response = await fetch(`${baseUrl}${path}`);
  if (!response.ok) {
    throw new Error(`API Error: ${response.status} ${response.statusText}`);
  }
  return response.json();
}

export async function apiPost<T>(path: string, data: any): Promise<T> {
  const response = await fetch(`${baseUrl}${path}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(errorText || `API Error: ${response.status} ${response.statusText}`);
  }
  return response.json();
}