// Task types for AI Content Platform

export interface Task {
  id: string;
  name: string;
  platform: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  content_count: number;
  created_at: string;
  updated_at: string;
}

export interface TaskCreate {
  name: string;
  prompt: string;
  platform: string;
  account_ids: string[];
  cron_expression?: string;
}

export type TaskUpdate = Partial<Omit<Task, 'id'>>;
