"use client"

import { useEffect, useState, useRef } from "react"

interface Order {
  data: string
  position: number
}

export default function ThroughputCard() {
  const [orders, setOrders] = useState<Order[]>([])
  const [isVisible, setIsVisible] = useState(false)
  const componentRef = useRef<HTMLDivElement>(null)

  // Intersection Observer to detect when component is in view
  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsVisible(entry.isIntersecting)
      },
      {
        threshold: 0.1, // Trigger when 30% of the component is visible
        rootMargin: "0px 0px -100px 0px", // Start animation slightly before fully in view
      },
    )

    if (componentRef.current) {
      observer.observe(componentRef.current)
    }

    return () => {
      if (componentRef.current) {
        observer.unobserve(componentRef.current)
      }
    }
  }, [])

  // Order generation effect - only runs when visible
  useEffect(() => {
    if (!isVisible) return

    const interval = setInterval(() => {
      const data = Array.from({ length: 20 }, () => Math.random().toString(36).substr(2))
        .join("")
        .substr(0, 100)
        .toUpperCase()

      setOrders((prev) => {
        const newOrder: Order = {
          data,
          position: 0,
        }

        // Move existing orders up and add new one at bottom
        const updatedOrders = prev
          .map((order) => ({ ...order, position: order.position + 1 }))
          .filter((order) => order.position < 24) // Keep only 24 orders

        return [newOrder, ...updatedOrders]
      })
    }, 33.3) // ~30 orders per second

    return () => clearInterval(interval)
  }, [isVisible])

  return (
    <div
      ref={componentRef}
      className={`md:col-span-2 h-96 bg-[#0e0e0e] border border-[rgba(38,38,38,.7)] rounded-[20px] p-8 flex flex-col`}
    >
      <div className="flex-grow flex items-center justify-center text-gray-500">
        <div className="relative w-full h-full overflow-hidden">
          {orders.map((order) => (
            <div
              key={order.data}
              className={`
                absolute left-1/2 w-fit
                ${order.position === 0 ? "text-cyan-400" : "text-blue-400/50"}
                ${order.position === 0 ? "border-t border-b" : ""}
                bg-blue-500/8 border-l border-r border-blue-400/15
                rounded px-3 py-1 text-xs font-mono
                backdrop-blur-sm transition-all duration-300 ease-out
              `}
              style={{
                bottom: `${20 + order.position * 10}px`,
                transform: `translateX(-50%) scale(${order.position === 0 ? 1 : order.position === 1 ? 0.95 : 0.9})`,
                opacity: order.position === 0 ? 1 : order.position === 1 ? 0.6 : 0.3,
                whiteSpace: "nowrap",
              }}
            >
              {order.data}
            </div>
          ))}

          {/* Main throughput display - centered with background */}
          <div className="relative z-10 flex items-center justify-center h-full">
            <div className="text-center bg-[#0e0e0e]/50 backdrop-blur-sm rounded-lg px-6 py-4 border border-gray-800/50">
              <div className="text-5xl font-light text-[#3CFFFF] mb-1 drop-shadow-lg">4,000,000</div>
              <div className="text-sm text-gray-400 uppercase tracking-widest">per second</div>
            </div>
          </div>
        </div>
      </div>

      <div className={`text-center`}>
        <h4 className="text-white font-medium text-xl mb-2">World-class throughput</h4>
        <p className="text-gray-400">Core matching engine is capable of processing four-million orders per second.</p>
      </div>
    </div>
  )
}
