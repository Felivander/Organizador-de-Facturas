# Organizador Ejecutivo de Archivos (Edición Rust 🦀)

Herramienta nativa para Windows de alto rendimiento diseñada para escanear, clasificar y ordenar automáticamente los archivos de tu carpeta de **Descargas** (o cualquier carpeta donde se ejecute), repartiéndolos en subdirectorios organizados por categoría, fecha y tipo.

Al estar escrito en **Rust**, el programa es extremadamente liviano (menos de 5MB) e instantáneo. ⚡

---

## 🚀 Cómo usarlo

1. Copiá el archivo ejecutable **`OrganizadorFacturas.exe`** y pegalo en la carpeta que quieras organizar (ej: *Descargas*).
2. Hacé **Doble Clic** para abrir la interfaz corporativa.
3. Presioná el botón **"Organizar Archivos"**.
4. ¡Listo! Verás el progreso en tiempo real. 

Si te arrepentís de una organización, el botón rojo **"↩ REVERTIR"** rastreará los archivos movidos, los devolverá a la raíz y limpiará las carpetas vacías sobrantes para que no quede basura.

---

## 🎨 Características de la Interfaz

- **Estética Empresarial**: Colores azul marino oscuro basados en la identidad visual de la empresa.
- **Diseño Inteligente**: 
  - La ventana se inicia compacta (**280px** de alto).
  - Al procesar, se expande automáticamente (**450px de alto y 550px de ancho**) revelando un panel inferior de logs.
- **Registro de Actividad**: Te resume qué departamentos/formatos sufrieron modificaciones en un listado de puntos limpios. El scrollbar está arrinconado contra el borde derecho absoluto de la ventana.

---

## 📂 Clasificación y Formato de Carpetas

Todas las carpetas de destino se crean en el disco de tu computadora precedidas por un círculo negro (**`⚫ `**) para agruparlas armónicamente en el Explorador de Archivos de Windows.

### 📄 1. Por Contenido (Nombres de PDFs)
Identifica patrones clave de la empresa en archivos `.pdf`:
- `⚫ Transferencias`
- `⚫ Depósitos`
- `⚫ Comprobantes Varios`
- `⚫ Planillas de Carga`
- `⚫ Detalles Cargas y Descargas`
- `⚫ Recibos de Clientes`
- `⚫ Caja y Bancos`
- `⚫ Liquidaciones`
- `⚫ Anticipos`
- `⚫ Remitos y Consignaciones`
- `⚫ Presupuestos y Minutas`
- `⚫ Recursos Humanos` (CVs, Curriculums)
- `⚫ Facturas y Notas` (`FC__`, `NC__`, Factura, etc)

> [!NOTE]
> Si el PDF posee un timestamp (`AAAAMMDDHHMMSS_...`), el programa crea dentro de la categoría otra subcarpeta por **Mes y Año** (ej: `Julio 2026`) y dentro de ella otra con el **Día** (`05`).

### 📦 2. Por Formato (Extensiones)
Para archivos que no sean PDFs, se categorizan directamente:
- `⚫ Imágenes`: `.jpg`, `.png`, `.gif`, `.webp`, `.svg`
- `⚫ Audio`: `.mp3`, `.wav`, `.ogg`, `.flac`
- `⚫ Videos`: `.mp4`, `.mkv`, `.avi`, `.mov`
- `⚫ Planillas Excel`: `.xls`, `.xlsx`, `.csv`, `.ods`, `.xlsm`
- `⚫ Presentaciones`: `.ppt`, `.pptx`
- `⚫ Archivos de Texto`: `.doc`, `.docx`, `.txt`, `.rtf`
- `⚫ Ejecutables`: `.exe`, `.msi`, `.bat`, `.cmd`
- `⚫ Comprimidos`: `.zip`, `.rar`, `.7z`, `.tar`, `.gz`
- `⚫ Ebooks`: `.epub`, `.mobi`, `.azw`
- `⚫ Documentos Varios`: PDFs genéricos sin coincidencia de reglas.

---

*Desarrollado de forma nativa para Windows.* 🖥️
