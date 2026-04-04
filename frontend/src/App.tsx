import { useState } from 'react';
import { TaskList } from './components/TaskList';
import { ContentList } from './components/ContentList';
import { AccountList } from './components/AccountList';
import { LogList } from './components/LogList';
import { CreateTaskModal } from './components/CreateTaskModal';
import { CreateAccountModal } from './components/CreateAccountModal';

type TabType = 'tasks' | 'contents' | 'accounts' | 'logs';

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('tasks');
  const [showCreateTask, setShowCreateTask] = useState(false);
  const [showCreateAccount, setShowCreateAccount] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);

  const refresh = () => setRefreshKey(k => k + 1);

  const tabs = [
    { id: 'tasks' as const, label: '任务管理', icon: '📋' },
    { id: 'contents' as const, label: '内容管理', icon: '📝' },
    { id: 'accounts' as const, label: '账号管理', icon: '👤' },
    { id: 'logs' as const, label: '日志查看', icon: '📜' },
  ];

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <h1 className="text-xl font-bold text-gray-900">
              🤖 AI 内容发布平台
            </h1>
            <div className="text-sm text-gray-500">
              Phase 6 - 前端管理面板
            </div>
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <nav className="flex space-x-8" aria-label="Tabs">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`${
                  activeTab === tab.id
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center gap-2`}
              >
                <span>{tab.icon}</span>
                {tab.label}
              </button>
            ))}
          </nav>
        </div>
      </div>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Action Buttons */}
        <div className="mb-6 flex justify-between items-center">
          <h2 className="text-lg font-medium text-gray-900">
            {tabs.find(t => t.id === activeTab)?.label}
          </h2>
          <div className="flex gap-3">
            {activeTab === 'tasks' && (
              <button
                onClick={() => setShowCreateTask(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
              >
                + 创建任务
              </button>
            )}
            {activeTab === 'accounts' && (
              <button
                onClick={() => setShowCreateAccount(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
              >
                + 添加账号
              </button>
            )}
            <button
              onClick={refresh}
              className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
            >
              🔄 刷新
            </button>
          </div>
        </div>

        {/* Content */}
        <div key={refreshKey}>
          {activeTab === 'tasks' && <TaskList onAction={refresh} />}
          {activeTab === 'contents' && <ContentList />}
          {activeTab === 'accounts' && <AccountList onAction={refresh} />}
          {activeTab === 'logs' && <LogList />}
        </div>
      </main>

      {/* Modals */}
      {showCreateTask && (
        <CreateTaskModal
          onClose={() => setShowCreateTask(false)}
          onSuccess={() => {
            setShowCreateTask(false);
            refresh();
          }}
        />
      )}
      {showCreateAccount && (
        <CreateAccountModal
          onClose={() => setShowCreateAccount(false)}
          onSuccess={() => {
            setShowCreateAccount(false);
            refresh();
          }}
        />
      )}
    </div>
  );
}

export default App;
