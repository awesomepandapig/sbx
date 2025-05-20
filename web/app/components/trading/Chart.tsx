import {
  createChart,
  ColorType,
  ISeriesApi,
  UTCTimestamp,
  CandlestickData,
  CandlestickSeriesOptions,
  CandlestickSeries,
  DeepPartial,
  Time, // Use Time type which covers string, number, BusinessDay
  CandlestickSeriesPartialOptions, // Correct type for addCandlestickSeries options
} from "lightweight-charts";
import { useEffect, useRef, useState } from "react";
import { WS_URL } from "~/lib/config"; // Assuming this path is correct

// --- Interfaces for WebSocket Data (adjust based on actual structure) ---
interface CandleUpdate {
  start: Time;
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
  product_id: string;
}

interface CandleEvent {
  product_id: string;
  candles: CandleUpdate[];
}

interface CandleMessage {
  channel: string;
  client_id: string; // Or other identifying fields
  timestamp: string;
  sequence_num: number;
  events: CandleEvent[];
}
// --- End Interfaces ---

interface ChartProps {
  symbol: string;
  // Optional: Add a prop to fetch initial data if needed
  // fetchInitialData: (symbol: string) => Promise<CandlestickData[]>;
}

// --- Default Candlestick Series Options (from example) ---
const candlestickSeriesOptions: DeepPartial<CandlestickSeriesPartialOptions> = {
  upColor: "#26a69a",
  downColor: "#ef5350",
  borderVisible: false, // Example uses false, adjust if you want borders
  wickUpColor: "#26a69a",
  wickDownColor: "#ef5350",
};

// Helper to safely convert Time to a comparable number (seconds timestamp)
// Returns -Infinity for invalid/unsupported types to ensure they are treated as "older"
const timeToComparable = (time: Time | undefined | null): number => {
  if (typeof time === "number") {
    return time; // Assumed UTCTimestamp (seconds)
  }
  if (typeof time === "string") {
    // Attempt to parse ISO string or YYYY-MM-DD
    const date = new Date(time);
    if (!isNaN(date.getTime())) {
      // If it's just YYYY-MM-DD, getTime() is UTC midnight. Divide by 1000 for seconds.
      return Math.floor(date.getTime() / 1000);
    }
  }
  // Handle BusinessDay if necessary, otherwise treat as invalid/very old
  // if (typeof time === 'object' && time.year && time.month && time.day) {
  //    Simple comparison might be tricky, convert to UTC seconds if possible
  // }
  return -Infinity; // Treat unparseable/unsupported times as infinitely old
};

export default function Chart({ symbol }: ChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<ReturnType<typeof createChart> | null>(null);
  const candlestickSeriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);
  // State to track if initial data has been loaded
  const [isInitialized, setIsInitialized] = useState(false);
  // Ref to store the timestamp of the *last* data point known to the series
  const lastDataPointTimeRef = useRef<Time | null>(null);

  // --- Effect for Chart Initialization and Resizing ---
  useEffect(() => {
    const container = chartContainerRef.current;
    if (!container || chartRef.current) return; // Prevent re-initialization

    // Calculate initial height
    const parentHeight = container.parentElement?.clientHeight ?? 400;
    const chartHeight = parentHeight > 60 ? parentHeight - 60 : 300;

    const chart = createChart(container, {
      // Using layout options similar to the example
      layout: {
        background: { type: ColorType.Solid, color: "#030105" }, // Example background
        textColor: "white", // Example text color
      },
      width: container.clientWidth,
      height: chartHeight,
      // Add other options you need (like timescale, grid, crosshair)
      rightPriceScale: {
        scaleMargins: { top: 0.3, bottom: 0.25 },
      },
      timeScale: {
        // timeVisible: true, // Often useful for intraday
        // secondsVisible: false, // Depending on candle granularity
      },
      crosshair: {
        mode: 1, // Example default mode is Magnet
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
    });
    chartRef.current = chart; // Store chart instance

    // Add Candlestick Series
    const candleSeries = chart.addSeries(
      CandlestickSeries,
      candlestickSeriesOptions,
    );
    candlestickSeriesRef.current = candleSeries;

    // --- !!! CRITICAL: Load Initial Historical Data Here !!! ---
    // You MUST load historical data before processing WebSocket updates
    // Otherwise, 'series.update()' won't work correctly initially.
    const fetchAndSetInitialData = async () => {
      try {
        // Replace with your actual API call to get historical candle data
        // The data should be an array of CandlestickData objects, sorted by time ascending.
        console.log(`Fetching initial data for ${symbol}...`);
        // Example: const response = await fetch(`/api/historical-candles?symbol=${symbol}&limit=500`);
        // const historicalData: CandlestickData[] = await response.json();

        // --- MOCK DATA (Replace with actual fetch) ---
        // Using the demo's generator logic *conceptually* for mock initial data
        const mockInitial = generateInitialMockData(200); // Generate ~200 historical candles
        console.log(`Setting ${mockInitial.length} initial candles.`);
        candlestickSeriesRef.current?.setData(mockInitial);
        // --- END MOCK DATA ---

        // Optional: Adjust the view after setting data
        chart.timeScale().fitContent();
        setIsInitialized(true); // Mark as initialized
        console.log(
          `Initial data set for ${symbol}. Ready for WebSocket updates.`,
        );
      } catch (error) {
        console.error("Failed to fetch or set initial chart data:", error);
        // Handle error appropriately (e.g., show message to user)
      }
    };

    fetchAndSetInitialData();
    // --- End Initial Data Loading ---

    // --- Resize Handler ---
    const handleResize = () => {
      if (container && chartRef.current) {
        const newParentHeight = container.parentElement?.clientHeight ?? 400;
        const newChartHeight =
          newParentHeight > 60 ? newParentHeight - 60 : 300;
        chartRef.current.applyOptions({
          width: container.clientWidth,
          height: newChartHeight,
        });
      }
    };
    window.addEventListener("resize", handleResize);

    // --- Cleanup ---
    return () => {
      console.log("Cleaning up chart...");
      window.removeEventListener("resize", handleResize);
      if (chartRef.current) {
        chartRef.current.remove();
        chartRef.current = null;
      }
      candlestickSeriesRef.current = null;
      setIsInitialized(false); // Reset initialization status
    };
  }, []); // Run only once on mount

  // --- Effect for WebSocket Connection and Data Handling ---
  useEffect(() => {
    // Only connect WebSocket *after* the chart and initial data are ready
    if (!isInitialized || !candlestickSeriesRef.current) {
      // console.log("Waiting for chart initialization before connecting WebSocket...");
      return;
    }

    console.log(
      `WebSocket effect running for ${symbol}. Initialized: ${isInitialized}`,
    );
    const ws = new WebSocket(WS_URL);
    const series = candlestickSeriesRef.current; // Capture ref value

    ws.onopen = () => {
      console.log(
        `WebSocket connected for ${symbol}. Subscribing to candles...`,
      );
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: [symbol],
          channel: "candles",
        }),
      );
    };

    ws.onmessage = (event) => {
      // Ensure series still exists (might be cleaned up during message processing)
      if (!series) return;

      try {
        const message: CandleMessage = JSON.parse(event.data);

        if (message?.channel !== "candles" || !message.events) {
          return; // Ignore irrelevant messages
        }

        message.events.forEach((ev) => {
          // Process updates only for the relevant symbol
          if (!ev.candles) return;

          ev.candles.forEach((candle) => {
            if (candle.product_id != symbol) return;

            // --- Data Transformation ---
            // Convert to CandlestickData format expected by lightweight-charts
            // CRITICAL: Ensure `update.start` is correctly interpreted as `Time`
            // (UTCTimestamp in seconds or 'YYYY-MM-DD'). Adjust if necessary.
            const candleData: CandlestickData = {
              time: candle.start, // Assuming update.start is already compatible
              open: Number(candle.open),
              high: Number(candle.high),
              low: Number(candle.low),
              close: Number(candle.close),
            };

            // console.log(candleData.time);
            // --- Update the Chart using series.update() ---
            // This will add or update the last candle based on the 'time' field
            series.update(candleData);
          });
        });
      } catch (error) {
        console.error(
          "Failed to parse WebSocket message or update chart:",
          error,
        );
      }
    };

    ws.onerror = (error) => {
      console.error(`WebSocket error for ${symbol}:`, error);
    };

    ws.onclose = (event) => {
      console.log(
        `WebSocket closed for ${symbol}. Code: ${event.code}, Reason: ${event.reason}`,
      );
      // Optional: Implement reconnection logic here if desired
    };

    // --- Cleanup WebSocket ---
    return () => {
      console.log(`Cleaning up WebSocket for ${symbol}`);
      if (ws && ws.readyState === WebSocket.OPEN) {
        // Optionally send unsubscribe message here if API supports it
        // ws.send(JSON.stringify({ type: "unsubscribe", ... }));
        ws.close(1000, "Component unmounting"); // Close gracefully
      } else if (ws) {
        // If ws exists but isn't open, attempt cleanup just in case
        ws.close();
      }
    };
    // Rerun this effect if the symbol changes OR when initialization completes
  }, [symbol, isInitialized]);

  // Helper function for MOCK initial data (replace with your actual fetch)
  function generateInitialMockData(numberOfCandles = 200): CandlestickData[] {
    const initialData: CandlestickData[] = [];
    let date = new Date(Date.now() - numberOfCandles * 24 * 60 * 60 * 1000); // Start N days ago
    let lastClose = 100 + Math.random() * 50;

    for (let i = 0; i < numberOfCandles; i++) {
      date.setUTCDate(date.getUTCDate() + 1); // Increment day
      const time = Math.floor(date.getTime() / 1000) as UTCTimestamp; // Use UTC seconds
      const open = lastClose + (Math.random() - 0.5) * 5;
      const close = open + (Math.random() - 0.5) * 10;
      const high = Math.max(open, close) + Math.random() * 5;
      const low = Math.min(open, close) - Math.random() * 5;
      lastClose = close;
      initialData.push({ time, open, high, low, close });
    }
    return initialData;
  }

  // The div that will contain the chart
  return (
    <div
      ref={chartContainerRef}
      style={{ width: "100%", height: "100%", position: "relative" }}
    />
  );
}
