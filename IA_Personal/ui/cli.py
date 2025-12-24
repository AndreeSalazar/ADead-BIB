"""
CLI para IA-Personal
=====================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent))


def main():
    """Punto de entrada principal de IA-Personal."""
    parser = argparse.ArgumentParser(
        description="🤖 IA-Personal - Tu asistente personal para ADead-BIB",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Ejemplos:
  python -m IA_Personal                    # Chat estándar
  python -m IA_Personal --turbo            # Modo rápido
  python -m IA_Personal --ollama           # Con Ollama LLM
  python -m IA_Personal --adead            # Con aceleración ADead-BIB
  python -m IA_Personal --full             # Todas las integraciones
  python -m IA_Personal --demo             # Demo del sistema
  python -m IA_Personal --benchmark        # Benchmark de rendimiento

Made with ❤️ in Peru 🇵🇪 by Eddi Andreé Salazar Matos
        """
    )
    
    # Modos de ejecución
    parser.add_argument("--turbo", action="store_true",
                        help="Modo turbo (más rápido, menos preciso)")
    parser.add_argument("--ollama", action="store_true",
                        help="Habilitar integración con Ollama LLM")
    parser.add_argument("--adead", action="store_true",
                        help="Habilitar aceleración ADead-BIB")
    parser.add_argument("--gpu", action="store_true",
                        help="Habilitar aceleración GPU (CUDA)")
    parser.add_argument("--gpu-max", action="store_true",
                        help="GPU MAX: Flash Attention + BF16 + Tensor Cores")
    parser.add_argument("--voice", action="store_true",
                        help="Control por voz (micrófono + comandos)")
    parser.add_argument("--full", action="store_true",
                        help="Habilitar todas las integraciones")
    
    # Acciones especiales
    parser.add_argument("--demo", action="store_true",
                        help="Ejecutar demo del sistema")
    parser.add_argument("--benchmark", action="store_true",
                        help="Ejecutar benchmark de rendimiento")
    parser.add_argument("--info", action="store_true",
                        help="Mostrar información del sistema")
    
    # Configuración
    parser.add_argument("--model", type=str, default="tinyllama",
                        help="Modelo de Ollama a usar (default: tinyllama)")
    
    args = parser.parse_args()
    
    # Determinar modo
    if args.full:
        mode = "full"
    elif getattr(args, 'voice', False):
        mode = "voice"
    elif getattr(args, 'gpu_max', False):
        mode = "gpu_max"
    elif args.gpu:
        mode = "gpu"
    elif args.ollama:
        mode = "ollama"
    elif args.adead:
        mode = "adead"
    else:
        mode = "standard"
    
    # Ejecutar acción
    if args.demo:
        run_demo(mode)
    elif args.benchmark:
        run_benchmark(mode)
    elif args.info:
        show_info()
    elif mode == "voice":
        # Modo voz
        from IA_Personal.voice.ia_personal_voice import IAPersonalVoice
        ia_voice = IAPersonalVoice(use_gpu=True, use_tts=True)
        ia_voice.start()
    else:
        # Chat interactivo
        from IA_Personal.ui.chat import IAPersonalChat
        chat = IAPersonalChat(mode=mode, turbo=args.turbo)
        chat.run()


def run_demo(mode: str = "standard"):
    """Ejecuta demo del sistema."""
    print("\n" + "=" * 60)
    print("   DEMO: IA-Personal para ADead-BIB")
    print("=" * 60)
    
    from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig
    
    if mode == "gpu_max" or mode == "full":
        from IA_Personal.integrations.ia_personal_gpu_max import IAPersonalGPUMax
        ia = IAPersonalGPUMax()
    elif mode == "gpu":
        from IA_Personal.integrations.gpu_compute import IAPersonalGPU
        ia = IAPersonalGPU()
    elif mode == "ollama":
        from IA_Personal.integrations.ollama_chat import IAPersonalOllama
        ia = IAPersonalOllama()
    elif mode == "adead":
        from IA_Personal.integrations.adead_accelerator import IAPersonalADead
        ia = IAPersonalADead()
    else:
        ia = IAPersonal()
    
    # Simular conversación
    print("\n📝 Simulación de Conversación:")
    print("-" * 40)
    
    messages = [
        "Hola",
        "Me llamo Developer",
        "Me gusta la programación y la IA",
        "Recuerda que estoy trabajando en ADead-BIB",
        "¿Qué sabes de mí?",
        "perfil",
        "memoria",
    ]
    
    import time
    for msg in messages:
        print(f"\n👤 Usuario: {msg}")
        start = time.time()
        response = ia.chat(msg)
        elapsed = (time.time() - start) * 1000
        print(f"🤖 IA: {response}")
        print(f"   ⏱️ {elapsed:.1f} ms")
        time.sleep(0.1)
    
    print("\n" + "=" * 60)
    print("   ✅ Demo Completada")
    stats = ia.get_stats()
    print(f"   💾 RAM: {stats['ram_mb']:.2f} MB")
    print(f"   📚 Memorias: {stats['memory_count']}")
    print("=" * 60)


def run_benchmark(mode: str = "standard"):
    """Ejecuta benchmark de rendimiento."""
    print("\n" + "=" * 60)
    print("   BENCHMARK: IA-Personal")
    print("=" * 60)
    
    import time
    import numpy as np
    
    from IA_Personal.core.ia_personal import IAPersonal, IAPersonalConfig
    
    config = IAPersonalConfig(
        vocab_size=10000,
        embed_dim=128,
        num_layers=2,
    )
    
    if mode == "gpu_max" or mode == "full":
        from IA_Personal.integrations.ia_personal_gpu_max import IAPersonalGPUMax
        ia = IAPersonalGPUMax(config)
        
        print("\n🔥 Benchmark GPU MAX:")
        print("-" * 40)
        ia.benchmark_full()
    elif mode == "gpu":
        from IA_Personal.integrations.gpu_compute import IAPersonalGPU
        ia = IAPersonalGPU(config)
        
        print("\n🎮 Benchmark GPU:")
        print("-" * 40)
        ia.benchmark_gpu()
    elif mode == "adead":
        from IA_Personal.integrations.adead_accelerator import IAPersonalADead
        ia = IAPersonalADead(config)
        
        print("\n⚡ Benchmark de Aceleración:")
        print("-" * 40)
        ia.benchmark_acceleration()
    else:
        ia = IAPersonal(config)
    
    # Benchmark de chat
    print("\n💬 Benchmark de Chat:")
    print("-" * 40)
    
    prompts = ["Hola", "¿Cómo estás?", "Cuéntame algo", "¿Qué puedes hacer?"]
    times = []
    
    for i in range(20):
        prompt = prompts[i % len(prompts)]
        start = time.time()
        ia.chat(prompt)
        times.append(time.time() - start)
    
    print(f"  Tiempo promedio: {np.mean(times)*1000:.1f} ms")
    print(f"  Tiempo mínimo:   {np.min(times)*1000:.1f} ms")
    print(f"  Tiempo máximo:   {np.max(times)*1000:.1f} ms")
    
    # Benchmark de memoria
    print("\n📚 Benchmark de Memoria:")
    print("-" * 40)
    
    start = time.time()
    for i in range(100):
        ia.memory.add(f"Test memory item {i}", category="general")
    add_time = time.time() - start
    
    start = time.time()
    for i in range(100):
        ia.memory.search(f"memory {i}", top_k=5)
    search_time = time.time() - start
    
    print(f"  Agregar 100 items: {add_time*1000:.1f} ms")
    print(f"  Buscar 100 veces:  {search_time*1000:.1f} ms")
    
    print("\n" + "=" * 60)
    print(f"   ✅ Benchmark Completado")
    if hasattr(ia.model, 'ram_mb'):
        print(f"   💾 RAM Total: {ia.model.ram_mb:.2f} MB")
    elif hasattr(ia.model, 'memory_mb'):
        print(f"   💾 RAM Total: {ia.model.memory_mb:.2f} MB")
    print("=" * 60)


def show_info():
    """Muestra información del sistema."""
    print("""
╔══════════════════════════════════════════════════════════════╗
║                    🤖 IA-Personal v1.0                        ║
║              Sistema de IA Personal para ADead-BIB            ║
╠══════════════════════════════════════════════════════════════╣
║                                                               ║
║  Author: Eddi Andreé Salazar Matos                           ║
║  Email:  eddi.salazar.dev@gmail.com                          ║
║  Made with ❤️ in Peru 🇵🇪                                     ║
║                                                               ║
╠══════════════════════════════════════════════════════════════╣
║  Características:                                             ║
║  • Memoria persistente entre sesiones                        ║
║  • Contexto personal (aprende de ti)                         ║
║  • Integración ADead-BIB (aceleración)                       ║
║  • Integración Ollama (LLM local)                            ║
║  • Ultra-ligero (~0.5 MB RAM)                                ║
║  • 100% privado (todo local)                                 ║
║                                                               ║
╠══════════════════════════════════════════════════════════════╣
║  Uso:                                                         ║
║  python -m IA_Personal           # Chat estándar             ║
║  python -m IA_Personal --ollama  # Con Ollama LLM            ║
║  python -m IA_Personal --adead   # Con aceleración           ║
║  python -m IA_Personal --demo    # Demo del sistema          ║
║                                                               ║
╚══════════════════════════════════════════════════════════════╝
    """)


if __name__ == "__main__":
    main()
