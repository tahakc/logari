"use client"

import * as React from "react"
import { useRouter } from "next/navigation"
import Link from "next/link"
import { createClient } from "@/lib/supabase/client"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

export type AuthFormMode = "login" | "signup"

interface AuthFormProps {
  mode: AuthFormMode
}

export function AuthForm({ mode }: AuthFormProps) {
  const router = useRouter()
  const [email, setEmail] = React.useState("")
  const [password, setPassword] = React.useState("")
  const [error, setError] = React.useState<string | null>(null)
  const [loading, setLoading] = React.useState(false)

  const isLogin = mode === "login"
  const title = isLogin ? "Login to your account" : "Create a new account"
  const buttonText = isLogin ? "Login" : "Sign up"
  const toggleText = isLogin
    ? "Don't have an account? Sign up"
    : "Already have an account? Login"
  const toggleLink = isLogin ? "/signup" : "/login"

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setLoading(true)

    try {
      const supabase = createClient()

      if (isLogin) {
        const { error } = await supabase.auth.signInWithPassword({
          email,
          password,
        })

        if (error) throw error
      } else {
        const { error } = await supabase.auth.signUp({
          email,
          password,
          options: {
            emailRedirectTo: `${window.location.origin}/auth/callback`,
          },
        })

        if (error) throw error
      }

      router.refresh()
      if (isLogin) {
        router.push("/dashboard")
      } else {
        // For signup, we can either redirect to a confirmation page or show a message
        setError("Check your email to confirm your account")
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="w-full max-w-md space-y-8 rounded-lg border border-border bg-card p-8 shadow-sm">
      <div className="space-y-2 text-center">
        <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
        <p className="text-sm text-muted-foreground">
          Enter your details below to {isLogin ? "login to" : "create"} your account
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            type="email"
            placeholder="name@example.com"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
            autoComplete={isLogin ? "username" : "email"}
          />
        </div>

        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <Label htmlFor="password">Password</Label>
            {isLogin && (
              <Link 
                href="/forgot-password" 
                className="text-xs text-primary hover:underline"
              >
                Forgot password?
              </Link>
            )}
          </div>
          <Input
            id="password"
            type="password"
            placeholder="••••••••"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            autoComplete={isLogin ? "current-password" : "new-password"}
          />
        </div>

        {error && (
          <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive">
            {error}
          </div>
        )}

        <Button type="submit" className="w-full" disabled={loading}>
          {loading ? "Loading..." : buttonText}
        </Button>

        <div className="text-center text-sm">
          <Link href={toggleLink} className="text-primary hover:underline">
            {toggleText}
          </Link>
        </div>
      </form>
    </div>
  )
} 
