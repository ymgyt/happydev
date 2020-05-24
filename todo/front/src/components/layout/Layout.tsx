import React from 'react';
import {AppBar, Toolbar, Typography, Paper} from '@material-ui/core';
import {fade, makeStyles} from '@material-ui/core/styles';
import SearchIcon from '@material-ui/icons/Search';
import InputBase from "@material-ui/core/InputBase";

const useStyles = makeStyles((theme) => ({
  paper: {
    padding:0,
    margin:0,
    backgroundColor: '#fafafa',
    fontFamily: 'Roboto',
  },
  search: {
    position: 'relative',
    borderRadius: '4px',
    backgroundColor: fade(theme.palette.common.white, 0.15),
    '&:hover': {
      backgroundColor: fade(theme.palette.common.white, 0.25),
    },
    marginRight: 16,
    marginLeft: 20,
    width: '30%',
  },
  searchIcon: {
    padding: '0 16px',
    height: '100%',
    position: 'absolute',
    pointerEvents: 'none',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  inputRoot: {
    color: 'inherit',
  },
  inputInput: {
    padding: '8px 8px 8px 0',
    // vertical padding + font size from searchIcon
    paddingLeft: `calc(1em + ${theme.spacing(4)}px)`,
    transition: theme.transitions.create('width'),
    width: '100%',
  },
}));

const Layout = (props: any) => {
  const classes = useStyles();
  return (
    <Paper
      elevation={0}
      variant='elevation'
      className={classes.paper}
    >
      <AppBar color='primary' position='static' style={{height: 64}}>
        <Toolbar style={{height: 64}}>
          <Typography variant='h6' color='inherit'>TODO</Typography>
          <div className={classes.search}>
            <div className={classes.searchIcon}>
              <SearchIcon/>
            </div>
            <InputBase
              placeholder='Search Tasks'
              classes={{
                root: classes.inputRoot,
                input: classes.inputInput,
              }}
            />
          </div>
        </Toolbar>
      </AppBar>
      {props.children}
    </Paper>
  )
};

export default Layout;
