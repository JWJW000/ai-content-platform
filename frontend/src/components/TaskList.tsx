import { useState, useEffect } from 'react';
import { tasksApi, type Task } from '../lib/api';

interface Props {
  onAction: () => void;
}

export function TaskList({ onAction }: Props) {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  useEffect(() => {
    loadTasks();
  }, []);

  const loadTasks = async () => {
    try {
      setLoading(true);
      const data = await tasksApi.list();
      setTasks(data);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : '加载失败');
    } finally {
      setLoading(false);
    }
  };

  const handleStart = async (id: string) => {
    try {
      setActionLoading(id);
      await tasksApi.start(id);
      await loadTasks();
      onAction();
    } catch (e) {
      alert(e instanceof Error ? e.message : '启动失败');
    } finally {
      setActionLoading(null);
    }
  };

  const handleStop = async (id: string) => {
    try {
      setActionLoading(id);
      await tasksApi.stop(id);
      await loadTasks();
      onAction();
    } catch (e) {
      alert(e instanceof Error ? e.message : '停止失败');
    } finally {
      setActionLoading(null);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('确定要删除这个任务吗？')) return;
    try {
      await tasksApi.delete(id);
      await loadTasks();
      onAction();
    } catch (e) {
      alert(e instanceof Error ? e.message : '删除失败');
    }
  };

  if (loading) {
    return <div className="text-center py-8 text-gray-500">加载中...</div>;
  }

  if (error) {
    return <div className="text-center py-8 text-red-500">{error}</div>;
  }

  if (tasks.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow p-8 text-center text-gray-500">
        暂无任务，点击右上角创建任务
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              任务名称
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              平台
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              状态
            </th>
            <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
              操作
            </th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {tasks.map((task) => (
            <tr key={task.id}>
              <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                {task.name}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {task.platform === 'xiaohongshu' ? '📕 小红书' : task.platform}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <span
                  className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                    task.status === 'running'
                      ? 'bg-green-100 text-green-800'
                      : 'bg-gray-100 text-gray-800'
                  }`}
                >
                  {task.status === 'running' ? '运行中' : '已停止'}
                </span>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <div className="flex justify-end gap-2">
                  {task.status === 'running' ? (
                    <button
                      onClick={() => handleStop(task.id)}
                      disabled={actionLoading === task.id}
                      className="text-orange-600 hover:text-orange-900 disabled:opacity-50"
                    >
                      {actionLoading === task.id ? '处理中...' : '⏹ 停止'}
                    </button>
                  ) : (
                    <button
                      onClick={() => handleStart(task.id)}
                      disabled={actionLoading === task.id}
                      className="text-green-600 hover:text-green-900 disabled:opacity-50"
                    >
                      {actionLoading === task.id ? '处理中...' : '▶ 启动'}
                    </button>
                  )}
                  <button
                    onClick={() => handleDelete(task.id)}
                    className="text-red-600 hover:text-red-900"
                  >
                    🗑 删除
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
