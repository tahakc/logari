import * as React from "react"
import { cn } from "@/lib/utils"

interface AuthCardProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode
}

export function AuthCard({ className, children, ...props }: AuthCardProps) {
  return (
    <div className="flex min-h-[80vh] items-center justify-center px-4 py-12">
      <div
        className={cn(
          "w-full max-w-md space-y-8 rounded-lg border bg-card p-8 shadow-sm",
          className
        )}
        {...props}
      >
        {children}
      </div>
    </div>
  )
}
