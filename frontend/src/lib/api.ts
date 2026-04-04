const API_BASE = 'http://localhost:8080/api';

interface ApiResponse<T> {
  code: number;
  data: T | null;
  message: string;
}

interface Task {
  id: string;
  name: string;
  platform: string;
  status: string;
}

interface Content {
  id: string;
  task_id: string;
  title: string;
  body: string;
  status: string;
  score: number | null;
  review_note: string | null;
  created_at: string;
  reviewed_at: string | null;
}

interface Account {
  id: string;
  platform: string;
  username: string;
  status: string;
}

interface Log {
  id: string;
  task_id: string | null;
  level: string;
  message: string;
  created_at: string;
}

async function fetchApi<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${url}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });
  const result: ApiResponse<T> = await response.json();
  if (result.code !== 0) {
    throw new Error(result.message);
  }
  return result.data as T;
}

// Tasks API
export const tasksApi = {
  list: () => fetchApi<Task[]>('/tasks'),
  
  create: (data: { name: string; platform: string; prompt: string; schedule: string }) =>
    fetchApi<Task>('/tasks', {
      method: 'POST',
      body: JSON.stringify(data),
    }),
  
  start: (id: string) =>
    fetchApi<null>(`/tasks/${id}/start`, { method: 'POST' }),
  
  stop: (id: string) =>
    fetchApi<null>(`/tasks/${id}/stop`, { method: 'POST' }),
  
  delete: (id: string) =>
    fetchApi<null>(`/tasks/${id}`, { method: 'DELETE' }),
};

// Contents API
export const contentsApi = {
  list: (taskId?: string) =>
    taskId
      ? fetchApi<Content[]>(`/contents?task_id=${taskId}`)
      : fetchApi<Content[]>('/contents'),
};

// Accounts API
export const accountsApi = {
  list: () => fetchApi<Account[]>('/accounts'),
  
  create: (data: { platform: string; username: string; auth: string }) =>
    fetchApi<Account>('/accounts', {
      method: 'POST',
      body: JSON.stringify(data),
    }),
  
  delete: (id: string) =>
    fetchApi<null>(`/accounts/${id}`, { method: 'DELETE' }),
};

// Logs API
export const logsApi = {
  list: () => fetchApi<Log[]>('/logs'),
  
  getByTask: (taskId: string) =>
    fetchApi<Log[]>(`/logs/${taskId}`),
};

export type { Task, Content, Account, Log };
