import {
  AreaSeries,
  createChart,
  ColorType,
  ISeriesApi,
  UTCTimestamp,
} from "lightweight-charts";
import { useEffect, useRef, useState } from "react";

interface ChartProps {
  symbol: string;
}

export default function Chart({ symbol }: ChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const areaSeriesRef = useRef<ISeriesApi<"Area"> | null>(null);
  const lastUpdateRef = useRef<number>(Date.now());

  useEffect(() => {
    const container = chartContainerRef.current;
    if (!container) return;

    const parentHeight = container.parentElement?.clientHeight ?? 400;
    const chartHeight = parentHeight - 60;

    const chart = createChart(container, {
      layout: {
        background: { type: ColorType.Solid, color: "black" },
        textColor: "white",
      },
      width: container.clientWidth,
      height: chartHeight,
    });

    chart.applyOptions({
      rightPriceScale: {
        scaleMargins: {
          top: 0.4,
          bottom: 0.15,
        },
      },
      layout: {
        attributionLogo: false,
      },
      crosshair: {
        horzLine: {
          visible: false,
          labelVisible: false,
        },
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
    });

    const areaSeries = chart.addSeries(AreaSeries, {
      topColor: "#2962FF",
      bottomColor: "rgba(41, 98, 255, 0.28)",
      lineColor: "#2962FF",
      lineWidth: 2,
      crosshairMarkerVisible: true,
    });

    areaSeriesRef.current = areaSeries;

    chart.timeScale().fitContent();

    const handleResize = () => {
      chart.applyOptions({ width: container.clientWidth });
    };

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      chart.remove();
    };
  }, []);

  // Push new point to chart every 1000ms
  // TODO: REPLACE WITH CANDLES DATA
  // useEffect(() => {
  //   const interval = setInterval(() => {
  //     if (!tickerData || !areaSeriesRef.current) return;

  //     const now = Math.floor(Date.now() / 1000) as UTCTimestamp;
  //     const price = tickerData.price;

  //     areaSeriesRef.current.update({ time: now, value: price });
  //   }, 100);

  //   return () => clearInterval(interval);
  // }, [tickerData]);

  return <div ref={chartContainerRef} />;
}
