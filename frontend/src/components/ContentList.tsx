import { useState, useEffect } from 'react';
import { contentsApi, type Content } from '../lib/api';

export function ContentList() {
  const [contents, setContents] = useState<Content[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [filter, setFilter] = useState<string>('all');
  const [reviewing, setReviewing] = useState<string | null>(null);

  useEffect(() => {
    loadContents();
  }, []);

  const loadContents = async () => {
    try {
      setLoading(true);
      const data = await contentsApi.list();
      setContents(data);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : '加载失败');
    } finally {
      setLoading(false);
    }
  };

  const handleReview = async (id: string, approved: boolean) => {
    try {
      setReviewing(id);
      await fetch(`http://localhost:8080/api/contents/${id}/review`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved, note: approved ? '审核通过' : '需要修改' }),
      });
      await loadContents();
    } catch (e) {
      alert(e instanceof Error ? e.message : '审核失败');
    } finally {
      setReviewing(null);
    }
  };

  const getStatusBadge = (status: string) => {
    const badges: Record<string, { bg: string; text: string; label: string }> = {
      pending_review: { bg: 'bg-yellow-100', text: 'text-yellow-800', label: '待审核' },
      generated: { bg: 'bg-blue-100', text: 'text-blue-800', label: '已生成' },
      approved: { bg: 'bg-green-100', text: 'text-green-800', label: '已通过' },
      rejected: { bg: 'bg-red-100', text: 'text-red-800', label: '已拒绝' },
      published: { bg: 'bg-purple-100', text: 'text-purple-800', label: '已发布' },
      failed: { bg: 'bg-gray-100', text: 'text-gray-800', label: '失败' },
    };
    const badge = badges[status] || badges.generated;
    return (
      <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${badge.bg} ${badge.text}`}>
        {badge.label}
      </span>
    );
  };

  const filteredContents = filter === 'all' 
    ? contents 
    : contents.filter(c => c.status === filter);

  if (loading) {
    return <div className="text-center py-8 text-gray-500">加载中...</div>;
  }

  if (error) {
    return <div className="text-center py-8 text-red-500">{error}</div>;
  }

  if (contents.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow p-8 text-center text-gray-500">
        暂无内容，启动任务后会自动生成内容
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      {/* Filter Tabs */}
      <div className="p-4 border-b border-gray-200 flex gap-2 flex-wrap">
        <button
          onClick={() => setFilter('all')}
          className={`px-3 py-1 text-sm rounded-full ${filter === 'all' ? 'bg-blue-100 text-blue-800' : 'bg-gray-100 text-gray-600'}`}
        >
          全部 ({contents.length})
        </button>
        <button
          onClick={() => setFilter('pending_review')}
          className={`px-3 py-1 text-sm rounded-full ${filter === 'pending_review' ? 'bg-yellow-100 text-yellow-800' : 'bg-gray-100 text-gray-600'}`}
        >
          待审核 ({contents.filter(c => c.status === 'pending_review').length})
        </button>
        <button
          onClick={() => setFilter('approved')}
          className={`px-3 py-1 text-sm rounded-full ${filter === 'approved' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'}`}
        >
          已通过 ({contents.filter(c => c.status === 'approved').length})
        </button>
        <button
          onClick={() => setFilter('published')}
          className={`px-3 py-1 text-sm rounded-full ${filter === 'published' ? 'bg-purple-100 text-purple-800' : 'bg-gray-100 text-gray-600'}`}
        >
          已发布 ({contents.filter(c => c.status === 'published').length})
        </button>
      </div>

      {/* Content List */}
      <div className="divide-y divide-gray-200">
        {filteredContents.map((content) => (
          <div key={content.id} className="p-4">
            <div className="flex items-start justify-between">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <h3 className="text-sm font-medium text-gray-900 truncate">
                    {content.title}
                  </h3>
                  {getStatusBadge(content.status)}
                  {content.score !== null && (
                    <span className="text-xs text-gray-500">
                      ⭐ {content.score.toFixed(1)}
                    </span>
                  )}
                </div>
                <div className="mt-1 text-xs text-gray-500">
                  ID: {content.id.slice(0, 8)}... | 任务: {content.task_id.slice(0, 8)}...
                </div>
              </div>
              <button
                onClick={() => setExpandedId(expandedId === content.id ? null : content.id)}
                className="ml-4 text-sm text-blue-600 hover:text-blue-800"
              >
                {expandedId === content.id ? '收起' : '展开'}
              </button>
            </div>

            {/* Expanded Content */}
            {expandedId === content.id && (
              <div className="mt-4">
                <div className="p-3 bg-gray-50 rounded-md">
                  <pre className="text-xs text-gray-700 whitespace-pre-wrap overflow-x-auto">
                    {content.body}
                  </pre>
                </div>

                {/* Review Actions - Only for pending_review status */}
                {content.status === 'pending_review' && (
                  <div className="mt-4 flex gap-2 justify-end">
                    <button
                      onClick={() => handleReview(content.id, false)}
                      disabled={reviewing === content.id}
                      className="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-red-700 bg-red-100 hover:bg-red-200 disabled:opacity-50"
                    >
                      {reviewing === content.id ? '处理中...' : '❌ 拒绝'}
                    </button>
                    <button
                      onClick={() => handleReview(content.id, true)}
                      disabled={reviewing === content.id}
                      className="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-green-700 bg-green-100 hover:bg-green-200 disabled:opacity-50"
                    >
                      {reviewing === content.id ? '处理中...' : '✅ 通过'}
                    </button>
                  </div>
                )}

                {/* Review Note */}
                {content.review_note && (
                  <div className="mt-2 text-xs text-gray-500">
                    审核备注: {content.review_note}
                  </div>
                )}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
