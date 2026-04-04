// Account types for AI Content Platform

export interface Account {
  id: string;
  platform: string;
  username: string;
  status: 'active' | 'inactive';
  created_at: string;
}
