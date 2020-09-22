import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter } from 'react-router-dom';
import CssBaseline from '@material-ui/core/CssBaseline';
import { ThemeProvider } from '@material-ui/styles';
import App from './App';
import theme from './theme';

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <BrowserRouter>
      <CssBaseline />
      <React.StrictMode>
        <App />
      </React.StrictMode>
    </BrowserRouter>
  </ThemeProvider>,
  document.getElementById('root'),
);
