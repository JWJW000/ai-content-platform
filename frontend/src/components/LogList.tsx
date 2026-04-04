import { useState, useEffect } from 'react';
import { logsApi, type Log } from '../lib/api';

export function LogList() {
  const [logs, setLogs] = useState<Log[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadLogs();
  }, []);

  const loadLogs = async () => {
    try {
      setLoading(true);
      const data = await logsApi.list();
      setLogs(data);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : '加载失败');
    } finally {
      setLoading(false);
    }
  };

  const getLevelBadge = (level: string) => {
    const badges: Record<string, { bg: string; text: string }> = {
      info: { bg: 'bg-blue-100', text: 'text-blue-800' },
      warn: { bg: 'bg-yellow-100', text: 'text-yellow-800' },
      error: { bg: 'bg-red-100', text: 'text-red-800' },
    };
    const badge = badges[level] || badges.info;
    return (
      <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${badge.bg} ${badge.text}`}>
        {level}
      </span>
    );
  };

  const formatTime = (time: string) => {
    return new Date(time).toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  if (loading) {
    return <div className="text-center py-8 text-gray-500">加载中...</div>;
  }

  if (error) {
    return <div className="text-center py-8 text-red-500">{error}</div>;
  }

  if (logs.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow p-8 text-center text-gray-500">
        暂无日志
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <div className="p-4 border-b border-gray-200">
        <div className="text-sm text-gray-500">
          共 {logs.length} 条日志
        </div>
      </div>
      <div className="divide-y divide-gray-200 max-h-96 overflow-y-auto">
        {logs.map((log) => (
          <div key={log.id} className="p-4">
            <div className="flex items-start gap-3">
              {getLevelBadge(log.level)}
              <div className="flex-1 min-w-0">
                <p className="text-sm text-gray-900 break-all">
                  {log.message}
                </p>
                <div className="mt-1 text-xs text-gray-500 flex gap-4">
                  <span>
                    {log.task_id ? `任务: ${log.task_id.slice(0, 8)}...` : '系统'}
                  </span>
                  <span>{formatTime(log.created_at)}</span>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
