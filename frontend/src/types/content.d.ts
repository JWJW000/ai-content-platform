// Content types for AI Content Platform

export interface Content {
  id: string;
  task_id: string;
  title: string;
  body: string;
  status: 'pending' | 'approved' | 'rejected' | 'published';
  platform: string;
  created_at: string;
}
