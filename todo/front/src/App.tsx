import React from 'react';
import Tasks from './components/tasks/Tasks';
import Config from "./Config";
import {Provider} from 'react-redux';
import store from './store';
import Layout from "./components/layout/Layout";

const App: React.FC<{}> = () => {

  React.useEffect(() => {
    console.log("initialize app...", Config);
  },[])

  return (
    <Provider store={store}>
      <Layout>
        <Tasks />
      </Layout>
    </Provider>
  );
}

export default App;
