import React from 'react';
import { Route, Switch, Redirect } from 'react-router';
import { Provider } from 'react-redux';
import Task from 'components/pages/Task';
import Tasks from './components/tasks/Tasks';
import Config from './Config';
import store from './store';
import Layout from './components/layout/Layout';

const App: React.FC<unknown> = () => {
  React.useEffect(() => {
    console.log('initialize app...', Config);
  }, []);

  const task = {
    id: 'xxx',
    title: 'dummy',
    content: 'content body...',
  };

  return (
    <Provider store={store}>
      <Layout>
        <Switch>
          <Route path="/tasks/:taskId">
            <Task task={task} />
          </Route>
          <Route path="/tasks">
            <Tasks />
          </Route>
          <Redirect push to="/tasks" />
        </Switch>
      </Layout>
    </Provider>
  );
};

export default App;
