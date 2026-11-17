import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { ButtonHTMLAttributes } from 'react';
const variants = { default: 'primary', secondary: 'secondary', destructive: 'danger', ghost: 'ghost' };
export function Button({ className, variant = 'default', ...props }: ButtonHTMLAttributes<HTMLButtonElement> & {
    variant?: keyof typeof variants;
}) { return <button className={twMerge(clsx('button', variants[variant], className))} {...props}/>; }
