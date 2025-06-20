const template = (
  <div 
    id={userId} 
    className={isActive ? 'active' : 'inactive'}
    style={{backgroundColor: theme.color, padding: '10px'}}
    disabled={!isEnabled}
    onClick={handleClick}
  >
    Dynamic content here
  </div>
);