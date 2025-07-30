import React from 'react';

interface BitcoinIconProps {
  size?: number;
  className?: string;
}

export default function BitcoinIcon({ size = 24, className = "" }: BitcoinIconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="currentColor"
      className={className}
    >
      <circle cx="12" cy="12" r="12" fill="currentColor"/>
      <path 
        d="M15.5 10.4c.2-1.4-.9-2.2-2.4-2.7l.5-1.9-1.2-.3-.5 1.9c-.3-.1-.6-.1-.9-.2l.5-1.9-1.2-.3-.5 1.9-.7-.2-1.7-.4-.3 1.3s.9.2.9.2c.5.1.6.4.6.6l-.6 2.4v.1l-.9 3.5c-.1.2-.2.4-.6.3 0 0-.9-.2-.9-.2l-.6 1.4 1.6.4.8.2-.5 1.9 1.2.3.5-1.9c.3.1.6.1.9.2l-.5 1.9 1.2.3.5-1.9c2.1.4 3.7.2 4.4-1.8.5-1.6-.3-2.5-1.2-3.1.9-.2 1.5-.8 1.7-2.1zm-3 4.2c-.4 1.4-2.9.7-3.7.5l.7-2.6c.8.2 3.4.6 3 2.1zm.4-4.2c-.3 1.3-2.4.6-3.1.4l.6-2.4c.7.2 2.9.5 2.5 2z" 
        fill="#000"
      />
    </svg>
  );
}

// import React from 'react';

// interface BitcoinIconProps {
//   size?: number;
//   className?: string;
// }

// export default function BitcoinIcon({ size = 24, className = "" }: BitcoinIconProps) {
//   return (
//     <svg
//       width={size}
//       height={size}
//       viewBox="0 0 24 24"
//       fill="currentColor"
//       className={className}
//     >
//       <circle cx="12" cy="12" r="12" fill="currentColor"/>
//       <path 
//         d="M15.5 10.4c.2-1.4-.9-2.2-2.4-2.7l.5-1.9-1.2-.3-.5 1.9c-.3-.1-.6-.1-.9-.2l.5-1.9-1.2-.3-.5 1.9-.7-.2-1.7-.4-.3 1.3s.9.2.9.2c.5.1.6.4.6.6l-.6 2.4v.1l-.9 3.5c-.1.2-.2.4-.6.3 0 0-.9-.2-.9-.2l-.6 1.4 1.6.4.8.2-.5 1.9 1.2.3.5-1.9c.3.1.6.1.9.2l-.5 1.9 1.2.3.5-1.9c2.1.4 3.7.2 4.4-1.8.5-1.6-.3-2.5-1.2-3.1.9-.2 1.5-.8 1.7-2.1zm-3 4.2c-.4 1.4-2.9.7-3.7.5l.7-2.6c.8.2 3.4.6 3 2.1zm.4-4.2c-.3 1.3-2.4.6-3.1.4l.6-2.4c.7.2 2.9.5 2.5 2z" 
//         fill="#fff"
//       />
//     </svg>
//   );
// }