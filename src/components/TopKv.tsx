"use client";

import Image from "next/image";
import React, { useEffect } from "react";

import { usePoints } from "@/hooks/usePoints";

import { InfoSingle } from "./Info";

function TopKv() {
  const { user, fetchPoints } = usePoints();

  useEffect(() => {
    fetchPoints();
    console.log("userpoints: ", user.points);
  }, [user, fetchPoints]);

  return (
    <div className="relative z-0 h-80 px-[30px] pb-[30px] pt-10">
      <div className="relative z-10 flex h-full flex-col justify-between">
        <div className="w-[220px] space-y-2">
          <h2>Enjoy &quot;Lossless & Fun&quot; Gaming</h2>
        </div>
        <InfoSingle title="Prize Pool" text="10,000,000" isButton={true} />
        <InfoSingle title="Your LUCK" text={user.points} isButton={false} />
      </div>
      <Image
        src="/top_kv_sp.png"
        fill={true}
        alt="Top Image"
        className="z-1 absolute left-0 top-0"
      />
    </div>
  );
}

export default TopKv;
