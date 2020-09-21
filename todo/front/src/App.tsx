import React from 'react';
import { Provider } from 'react-redux';
import Tasks from './components/tasks/Tasks';
import Config from './Config';
import store from './store';
import Layout from './components/layout/Layout';

const App: React.FC<unknown> = () => {
  React.useEffect(() => {
    console.log('initialize app...', Config);
  }, []);

  return (
    <Provider store={store}>
      <Layout>
        <Tasks />
      </Layout>
    </Provider>
  );
};

export default App;
