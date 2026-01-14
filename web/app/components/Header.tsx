export default function Header() {
  return (
    <header className="bg-gray-100 border-b border-gray-300 py-3">
      <div
        className="flex items-center justify-between gap-[2vw]"
        style={{ paddingLeft: "2vw", paddingRight: "2vw" }}
      >
        <div style={{ width: "20vw" }}></div>
        <div
          className="flex flex-col items-center"
          style={{ width: "min(50vw, calc(100vh - 7rem - 3vw))" }}
        >
          <h1 className="text-4xl font-bold text-gray-900">ESCAPADE</h1>
          <p className="text-base text-gray-600">
            An engine for Quarto written by Jonathan Roth
          </p>
        </div>
        <div style={{ width: "20vw" }}></div>
      </div>
    </header>
  );
}
