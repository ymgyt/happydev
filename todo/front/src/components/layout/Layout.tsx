import React from 'react';
import {AppBar, Toolbar, Typography, Paper, Box, Link} from '@material-ui/core';
import {fade, makeStyles} from '@material-ui/core/styles';
import SearchIcon from '@material-ui/icons/Search';
import {ReactComponent as GithubLogo} from '../../assets/github-brands.svg';
import SvgIcon from '@material-ui/core/SvgIcon';
import SearchTask from '../tasks/SearchTask';

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
          <SearchTask />
          </div>
        </Toolbar>
      </AppBar>
      {props.children}
      <Box>
        <Typography variant="body2" color="textSecondary" align="right" style={{marginRight: '16px'}}>
          <Link color="inherit" href='https://github.com/ymgyt/happydev/tree/master/todo'>
            <SvgIcon style={{display: 'inline-block', textAlign: 'center'}}><GithubLogo /></SvgIcon>
          </Link>
        </Typography>
      </Box>
    </Paper>
  )
};

export default Layout;
