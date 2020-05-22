import React,{ useState } from 'react';
import Tasks  from './components/tasks/Tasks';

function App() {
  const [name, setName] = useState<string | null>('ymgyt');

  return (
      <Tasks message={`${name}`}/>
  );
}

export default App;
