// sf::Font implementation bundled into a single file to ease conditional inclusion (text feature)

#include "Graphics/Rect.hpp"
#include <cstddef>
#include <cstdint>

////////////////////////////////////////////////////////////
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2023 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//
////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////
// Headers
////////////////////////////////////////////////////////////
#include <SFML/Graphics/Export.hpp>
#include <SFML/Graphics/Glyph.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <SFML/Graphics/Rect.hpp>
#include <map>
#include <string>
#include <vector>

namespace sf
{
class InputStream;

////////////////////////////////////////////////////////////
/// \brief Class for loading and manipulating character fonts
///
////////////////////////////////////////////////////////////
class SFML_GRAPHICS_API Font
{
public:

    ////////////////////////////////////////////////////////////
    /// \brief Holds various information about a font
    ///
    ////////////////////////////////////////////////////////////
    struct Info
    {
        std::string family; //!< The font family
    };

public:

    ////////////////////////////////////////////////////////////
    /// \brief Default constructor
    ///
    /// This constructor defines an empty font
    ///
    ////////////////////////////////////////////////////////////
    Font();

    ////////////////////////////////////////////////////////////
    /// \brief Copy constructor
    ///
    /// \param copy Instance to copy
    ///
    ////////////////////////////////////////////////////////////
    Font(const Font& copy);

    ////////////////////////////////////////////////////////////
    /// \brief Destructor
    ///
    /// Cleans up all the internal resources used by the font
    ///
    ////////////////////////////////////////////////////////////
    ~Font();

    ////////////////////////////////////////////////////////////
    /// \brief Load the font from a file
    ///
    /// The supported font formats are: TrueType, Type 1, CFF,
    /// OpenType, SFNT, X11 PCF, Windows FNT, BDF, PFR and Type 42.
    /// Note that this function knows nothing about the standard
    /// fonts installed on the user's system, thus you can't
    /// load them directly.
    ///
    /// \warning SFML cannot preload all the font data in this
    /// function, so the file has to remain accessible until
    /// the sf::Font object loads a new font or is destroyed.
    ///
    /// \param filename Path of the font file to load
    ///
    /// \return True if loading succeeded, false if it failed
    ///
    /// \see loadFromMemory, loadFromStream
    ///
    ////////////////////////////////////////////////////////////
    bool loadFromFile(const std::string& filename);

    ////////////////////////////////////////////////////////////
    /// \brief Load the font from a file in memory
    ///
    /// The supported font formats are: TrueType, Type 1, CFF,
    /// OpenType, SFNT, X11 PCF, Windows FNT, BDF, PFR and Type 42.
    ///
    /// \warning SFML cannot preload all the font data in this
    /// function, so the buffer pointed by \a data has to remain
    /// valid until the sf::Font object loads a new font or
    /// is destroyed.
    ///
    /// \param data        Pointer to the file data in memory
    /// \param sizeInBytes Size of the data to load, in bytes
    ///
    /// \return True if loading succeeded, false if it failed
    ///
    /// \see loadFromFile, loadFromStream
    ///
    ////////////////////////////////////////////////////////////
    bool loadFromMemory(const void* data, std::size_t sizeInBytes);

    ////////////////////////////////////////////////////////////
    /// \brief Load the font from a custom stream
    ///
    /// The supported font formats are: TrueType, Type 1, CFF,
    /// OpenType, SFNT, X11 PCF, Windows FNT, BDF, PFR and Type 42.
    /// Warning: SFML cannot preload all the font data in this
    /// function, so the contents of \a stream have to remain
    /// valid as long as the font is used.
    ///
    /// \warning SFML cannot preload all the font data in this
    /// function, so the stream has to remain accessible until
    /// the sf::Font object loads a new font or is destroyed.
    ///
    /// \param stream Source stream to read from
    ///
    /// \return True if loading succeeded, false if it failed
    ///
    /// \see loadFromFile, loadFromMemory
    ///
    ////////////////////////////////////////////////////////////
    bool loadFromStream(InputStream& stream);

    ////////////////////////////////////////////////////////////
    /// \brief Get the font information
    ///
    /// \return A structure that holds the font information
    ///
    ////////////////////////////////////////////////////////////
    const Info& getInfo() const;

    ////////////////////////////////////////////////////////////
    /// \brief Retrieve a glyph of the font
    ///
    /// If the font is a bitmap font, not all character sizes
    /// might be available. If the glyph is not available at the
    /// requested size, an empty glyph is returned.
    ///
    /// You may want to use \ref hasGlyph to determine if the
    /// glyph exists before requesting it. If the glyph does not
    /// exist, a font specific default is returned.
    ///
    /// Be aware that using a negative value for the outline
    /// thickness will cause distorted rendering.
    ///
    /// \param codePoint        Unicode code point of the character to get
    /// \param characterSize    Reference character size
    /// \param bold             Retrieve the bold version or the regular one?
    /// \param outlineThickness Thickness of outline (when != 0 the glyph will not be filled)
    ///
    /// \return The glyph corresponding to \a codePoint and \a characterSize
    ///
    ////////////////////////////////////////////////////////////
    const Glyph& getGlyph(Uint32 codePoint, unsigned int characterSize, bool bold, float outlineThickness = 0) const;

    ////////////////////////////////////////////////////////////
    /// \brief Determine if this font has a glyph representing the requested code point
    ///
    /// Most fonts only include a very limited selection of glyphs from
    /// specific Unicode subsets, like Latin, Cyrillic, or Asian characters.
    ///
    /// While code points without representation will return a font specific
    /// default character, it might be useful to verify whether specific
    /// code points are included to determine whether a font is suited
    /// to display text in a specific language.
    ///
    /// \param codePoint Unicode code point to check
    ///
    /// \return True if the codepoint has a glyph representation, false otherwise
    ///
    ////////////////////////////////////////////////////////////
    bool hasGlyph(Uint32 codePoint) const;

    ////////////////////////////////////////////////////////////
    /// \brief Get the kerning offset of two glyphs
    ///
    /// The kerning is an extra offset (negative) to apply between two
    /// glyphs when rendering them, to make the pair look more "natural".
    /// For example, the pair "AV" have a special kerning to make them
    /// closer than other characters. Most of the glyphs pairs have a
    /// kerning offset of zero, though.
    ///
    /// \param first         Unicode code point of the first character
    /// \param second        Unicode code point of the second character
    /// \param characterSize Reference character size
    ///
    /// \return Kerning value for \a first and \a second, in pixels
    ///
    ////////////////////////////////////////////////////////////
    float getKerning(Uint32 first, Uint32 second, unsigned int characterSize, bool bold = false) const;

    ////////////////////////////////////////////////////////////
    /// \brief Get the line spacing
    ///
    /// Line spacing is the vertical offset to apply between two
    /// consecutive lines of text.
    ///
    /// \param characterSize Reference character size
    ///
    /// \return Line spacing, in pixels
    ///
    ////////////////////////////////////////////////////////////
    float getLineSpacing(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    /// \brief Get the position of the underline
    ///
    /// Underline position is the vertical offset to apply between the
    /// baseline and the underline.
    ///
    /// \param characterSize Reference character size
    ///
    /// \return Underline position, in pixels
    ///
    /// \see getUnderlineThickness
    ///
    ////////////////////////////////////////////////////////////
    float getUnderlinePosition(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    /// \brief Get the thickness of the underline
    ///
    /// Underline thickness is the vertical size of the underline.
    ///
    /// \param characterSize Reference character size
    ///
    /// \return Underline thickness, in pixels
    ///
    /// \see getUnderlinePosition
    ///
    ////////////////////////////////////////////////////////////
    float getUnderlineThickness(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    /// \brief Retrieve the texture containing the loaded glyphs of a certain size
    ///
    /// The contents of the returned texture changes as more glyphs
    /// are requested, thus it is not very relevant. It is mainly
    /// used internally by sf::Text.
    ///
    /// \param characterSize Reference character size
    ///
    /// \return Texture containing the glyphs of the requested size
    ///
    ////////////////////////////////////////////////////////////
    const Texture& getTexture(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    /// \brief Enable or disable the smooth filter
    ///
    /// When the filter is activated, the font appears smoother
    /// so that pixels are less noticeable. However if you want
    /// the font to look exactly the same as its source file,
    /// you should disable it.
    /// The smooth filter is enabled by default.
    ///
    /// \param smooth True to enable smoothing, false to disable it
    ///
    /// \see isSmooth
    ///
    ////////////////////////////////////////////////////////////
    void setSmooth(bool smooth);

    ////////////////////////////////////////////////////////////
    /// \brief Tell whether the smooth filter is enabled or not
    ///
    /// \return True if smoothing is enabled, false if it is disabled
    ///
    /// \see setSmooth
    ///
    ////////////////////////////////////////////////////////////
    bool isSmooth() const;

    ////////////////////////////////////////////////////////////
    /// \brief Overload of assignment operator
    ///
    /// \param right Instance to assign
    ///
    /// \return Reference to self
    ///
    ////////////////////////////////////////////////////////////
    Font& operator =(const Font& right);

private:

    ////////////////////////////////////////////////////////////
    /// \brief Structure defining a row of glyphs
    ///
    ////////////////////////////////////////////////////////////
    struct Row
    {
        Row(unsigned int rowTop, unsigned int rowHeight) : width(0), top(rowTop), height(rowHeight) {}

        unsigned int width;  //!< Current width of the row
        unsigned int top;    //!< Y position of the row into the texture
        unsigned int height; //!< Height of the row
    };

    ////////////////////////////////////////////////////////////
    // Types
    ////////////////////////////////////////////////////////////
    typedef std::map<Uint64, Glyph> GlyphTable; //!< Table mapping a codepoint to its glyph

    ////////////////////////////////////////////////////////////
    /// \brief Structure defining a page of glyphs
    ///
    ////////////////////////////////////////////////////////////
    struct Page
    {
        explicit Page(bool smooth);

        GlyphTable       glyphs;  //!< Table mapping code points to their corresponding glyph
        Texture          texture; //!< Texture containing the pixels of the glyphs
        unsigned int     nextRow; //!< Y position of the next new row in the texture
        std::vector<Row> rows;    //!< List containing the position of all the existing rows
    };

    ////////////////////////////////////////////////////////////
    /// \brief Free all the internal resources
    ///
    ////////////////////////////////////////////////////////////
    void cleanup();

    ////////////////////////////////////////////////////////////
    /// \brief Find or create the glyphs page corresponding to the given character size
    ///
    /// \param characterSize Reference character size
    ///
    /// \return The glyphs page corresponding to \a characterSize
    ///
    ////////////////////////////////////////////////////////////
    Page& loadPage(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    /// \brief Load a new glyph and store it in the cache
    ///
    /// \param codePoint        Unicode code point of the character to load
    /// \param characterSize    Reference character size
    /// \param bold             Retrieve the bold version or the regular one?
    /// \param outlineThickness Thickness of outline (when != 0 the glyph will not be filled)
    ///
    /// \return The glyph corresponding to \a codePoint and \a characterSize
    ///
    ////////////////////////////////////////////////////////////
    Glyph loadGlyph(Uint32 codePoint, unsigned int characterSize, bool bold, float outlineThickness) const;

    ////////////////////////////////////////////////////////////
    /// \brief Find a suitable rectangle within the texture for a glyph
    ///
    /// \param page   Page of glyphs to search in
    /// \param width  Width of the rectangle
    /// \param height Height of the rectangle
    ///
    /// \return Found rectangle within the texture
    ///
    ////////////////////////////////////////////////////////////
    IntRect findGlyphRect(Page& page, unsigned int width, unsigned int height) const;

    ////////////////////////////////////////////////////////////
    /// \brief Make sure that the given size is the current one
    ///
    /// \param characterSize Reference character size
    ///
    /// \return True on success, false if any error happened
    ///
    ////////////////////////////////////////////////////////////
    bool setCurrentSize(unsigned int characterSize) const;

    ////////////////////////////////////////////////////////////
    // Types
    ////////////////////////////////////////////////////////////
    typedef std::map<unsigned int, Page> PageTable; //!< Table mapping a character size to its page (texture)

    ////////////////////////////////////////////////////////////
    // Member data
    ////////////////////////////////////////////////////////////
    void*                      m_library;     //!< Pointer to the internal library interface (it is typeless to avoid exposing implementation details)
    void*                      m_face;        //!< Pointer to the internal font face (it is typeless to avoid exposing implementation details)
    void*                      m_streamRec;   //!< Pointer to the stream rec instance (it is typeless to avoid exposing implementation details)
    void*                      m_stroker;     //!< Pointer to the stroker (it is typeless to avoid exposing implementation details)
    int*                       m_refCount;    //!< Reference counter used by implicit sharing
    bool                       m_isSmooth;    //!< Status of the smooth filter
    Info                       m_info;        //!< Information about the font
    mutable PageTable          m_pages;       //!< Table containing the glyphs pages by character size
    mutable std::vector<Uint8> m_pixelBuffer; //!< Pixel buffer holding a glyph's pixels before being written to the texture
    #ifdef SFML_SYSTEM_ANDROID
    void*                      m_stream; //!< Asset file streamer (if loaded from file)
    #endif
};

} // namespace sf

// == Taken from SFML's Font.cpp ==

#include <SFML/System/Err.hpp>
#include "ft2build.h"
#include FT_FREETYPE_H
#include FT_GLYPH_H
#include FT_OUTLINE_H
#include FT_BITMAP_H
#include FT_STROKER_H
#include <cstdlib>
#include <cstring>
#include <cmath>

namespace
{
    // Helper to intepret memory as a specific type
    template <typename T, typename U>
    inline T reinterpret(const U& input)
    {
        T output;
        std::memcpy(&output, &input, sizeof(U));
        return output;
    }

    // Combine outline thickness, boldness and font glyph index into a single 64-bit key
    sf::Uint64 combine(float outlineThickness, bool bold, sf::Uint32 index)
    {
        return (static_cast<sf::Uint64>(reinterpret<sf::Uint32>(outlineThickness)) << 32) | (static_cast<sf::Uint64>(bold) << 31) | index;
    }
}


namespace sf
{
////////////////////////////////////////////////////////////
Font::Font() :
m_library  (NULL),
m_face     (NULL),
m_streamRec(NULL),
m_stroker  (NULL),
m_refCount (NULL),
m_isSmooth (true),
m_info     ()
{
    #ifdef SFML_SYSTEM_ANDROID
        m_stream = NULL;
    #endif
}


////////////////////////////////////////////////////////////
Font::Font(const Font& copy) :
m_library    (copy.m_library),
m_face       (copy.m_face),
m_streamRec  (copy.m_streamRec),
m_stroker    (copy.m_stroker),
m_refCount   (copy.m_refCount),
m_isSmooth   (copy.m_isSmooth),
m_info       (copy.m_info),
m_pages      (copy.m_pages),
m_pixelBuffer(copy.m_pixelBuffer)
{
    #ifdef SFML_SYSTEM_ANDROID
        m_stream = NULL;
    #endif

    // Note: as FreeType doesn't provide functions for copying/cloning,
    // we must share all the FreeType pointers

    if (m_refCount)
        (*m_refCount)++;
}


////////////////////////////////////////////////////////////
Font::~Font()
{
    cleanup();
}


////////////////////////////////////////////////////////////
bool Font::loadFromFile(const std::string& filename)
{
    // Cleanup the previous resources
    cleanup();
    m_refCount = new int(1);

    // Initialize FreeType
    // Note: we initialize FreeType for every font instance in order to avoid having a single
    // global manager that would create a lot of issues regarding creation and destruction order.
    FT_Library library;
    if (FT_Init_FreeType(&library) != 0)
    {
        err() << "Failed to load font \"" << filename << "\" (failed to initialize FreeType)" << std::endl;
        return false;
    }
    m_library = library;

    // Load the new font face from the specified file
    FT_Face face;
    if (FT_New_Face(static_cast<FT_Library>(m_library), filename.c_str(), 0, &face) != 0)
    {
        err() << "Failed to load font \"" << filename << "\" (failed to create the font face)" << std::endl;
        return false;
    }

    // Load the stroker that will be used to outline the font
    FT_Stroker stroker;
    if (FT_Stroker_New(static_cast<FT_Library>(m_library), &stroker) != 0)
    {
        err() << "Failed to load font \"" << filename << "\" (failed to create the stroker)" << std::endl;
        FT_Done_Face(face);
        return false;
    }

    // Select the unicode character map
    if (FT_Select_Charmap(face, FT_ENCODING_UNICODE) != 0)
    {
        err() << "Failed to load font \"" << filename << "\" (failed to set the Unicode character set)" << std::endl;
        FT_Stroker_Done(stroker);
        FT_Done_Face(face);
        return false;
    }

    // Store the loaded font in our ugly void* :)
    m_stroker = stroker;
    m_face = face;

    // Store the font information
    m_info.family = face->family_name ? face->family_name : std::string();

    return true;
}


////////////////////////////////////////////////////////////
bool Font::loadFromMemory(const void* data, std::size_t sizeInBytes)
{
    // Cleanup the previous resources
    cleanup();
    m_refCount = new int(1);

    // Initialize FreeType
    // Note: we initialize FreeType for every font instance in order to avoid having a single
    // global manager that would create a lot of issues regarding creation and destruction order.
    FT_Library library;
    if (FT_Init_FreeType(&library) != 0)
    {
        err() << "Failed to load font from memory (failed to initialize FreeType)" << std::endl;
        return false;
    }
    m_library = library;

    // Load the new font face from the specified file
    FT_Face face;
    if (FT_New_Memory_Face(static_cast<FT_Library>(m_library), reinterpret_cast<const FT_Byte*>(data), static_cast<FT_Long>(sizeInBytes), 0, &face) != 0)
    {
        err() << "Failed to load font from memory (failed to create the font face)" << std::endl;
        return false;
    }

    // Load the stroker that will be used to outline the font
    FT_Stroker stroker;
    if (FT_Stroker_New(static_cast<FT_Library>(m_library), &stroker) != 0)
    {
        err() << "Failed to load font from memory (failed to create the stroker)" << std::endl;
        FT_Done_Face(face);
        return false;
    }

    // Select the Unicode character map
    if (FT_Select_Charmap(face, FT_ENCODING_UNICODE) != 0)
    {
        err() << "Failed to load font from memory (failed to set the Unicode character set)" << std::endl;
        FT_Stroker_Done(stroker);
        FT_Done_Face(face);
        return false;
    }

    // Store the loaded font in our ugly void* :)
    m_stroker = stroker;
    m_face = face;

    // Store the font information
    m_info.family = face->family_name ? face->family_name : std::string();

    return true;
}

////////////////////////////////////////////////////////////
const Font::Info& Font::getInfo() const
{
    return m_info;
}


////////////////////////////////////////////////////////////
const Glyph& Font::getGlyph(Uint32 codePoint, unsigned int characterSize, bool bold, float outlineThickness) const
{
    // Get the page corresponding to the character size
    GlyphTable& glyphs = loadPage(characterSize).glyphs;

    // Build the key by combining the glyph index (based on code point), bold flag, and outline thickness
    Uint64 key = combine(outlineThickness, bold, FT_Get_Char_Index(static_cast<FT_Face>(m_face), codePoint));

    // Search the glyph into the cache
    GlyphTable::const_iterator it = glyphs.find(key);
    if (it != glyphs.end())
    {
        // Found: just return it
        return it->second;
    }
    else
    {
        // Not found: we have to load it
        Glyph glyph = loadGlyph(codePoint, characterSize, bold, outlineThickness);
        return glyphs.insert(std::make_pair(key, glyph)).first->second;
    }
}


////////////////////////////////////////////////////////////
bool Font::hasGlyph(Uint32 codePoint) const
{
    return FT_Get_Char_Index(static_cast<FT_Face>(m_face), codePoint) != 0;
}


////////////////////////////////////////////////////////////
float Font::getKerning(Uint32 first, Uint32 second, unsigned int characterSize, bool bold) const
{
    // Special case where first or second is 0 (null character)
    if (first == 0 || second == 0)
        return 0.f;

    FT_Face face = static_cast<FT_Face>(m_face);

    if (face && setCurrentSize(characterSize))
    {
        // Convert the characters to indices
        FT_UInt index1 = FT_Get_Char_Index(face, first);
        FT_UInt index2 = FT_Get_Char_Index(face, second);

        // Retrieve position compensation deltas generated by FT_LOAD_FORCE_AUTOHINT flag
        float firstRsbDelta = static_cast<float>(getGlyph(first, characterSize, bold).rsbDelta);
        float secondLsbDelta = static_cast<float>(getGlyph(second, characterSize, bold).lsbDelta);

        // Get the kerning vector if present
        FT_Vector kerning;
        kerning.x = kerning.y = 0;
        if (FT_HAS_KERNING(face))
            FT_Get_Kerning(face, index1, index2, FT_KERNING_UNFITTED, &kerning);

        // X advance is already in pixels for bitmap fonts
        if (!FT_IS_SCALABLE(face))
            return static_cast<float>(kerning.x);

        // Combine kerning with compensation deltas and return the X advance
        // Flooring is required as we use FT_KERNING_UNFITTED flag which is not quantized in 64 based grid
        return std::floor((secondLsbDelta - firstRsbDelta + static_cast<float>(kerning.x) + 32) / static_cast<float>(1 << 6));
    }
    else
    {
        // Invalid font
        return 0.f;
    }
}


////////////////////////////////////////////////////////////
float Font::getLineSpacing(unsigned int characterSize) const
{
    FT_Face face = static_cast<FT_Face>(m_face);

    if (face && setCurrentSize(characterSize))
    {
        return static_cast<float>(face->size->metrics.height) / static_cast<float>(1 << 6);
    }
    else
    {
        return 0.f;
    }
}


////////////////////////////////////////////////////////////
float Font::getUnderlinePosition(unsigned int characterSize) const
{
    FT_Face face = static_cast<FT_Face>(m_face);

    if (face && setCurrentSize(characterSize))
    {
        // Return a fixed position if font is a bitmap font
        if (!FT_IS_SCALABLE(face))
            return static_cast<float>(characterSize) / 10.f;

        return -static_cast<float>(FT_MulFix(face->underline_position, face->size->metrics.y_scale)) / static_cast<float>(1 << 6);
    }
    else
    {
        return 0.f;
    }
}


////////////////////////////////////////////////////////////
float Font::getUnderlineThickness(unsigned int characterSize) const
{
    FT_Face face = static_cast<FT_Face>(m_face);

    if (face && setCurrentSize(characterSize))
    {
        // Return a fixed thickness if font is a bitmap font
        if (!FT_IS_SCALABLE(face))
            return static_cast<float>(characterSize) / 14.f;

        return static_cast<float>(FT_MulFix(face->underline_thickness, face->size->metrics.y_scale)) / static_cast<float>(1 << 6);
    }
    else
    {
        return 0.f;
    }
}


////////////////////////////////////////////////////////////
const Texture& Font::getTexture(unsigned int characterSize) const
{
    return loadPage(characterSize).texture;
}

////////////////////////////////////////////////////////////
void Font::setSmooth(bool smooth)
{
    if (smooth != m_isSmooth)
    {
        m_isSmooth = smooth;

        for (sf::Font::PageTable::iterator page = m_pages.begin(); page != m_pages.end(); ++page)
        {
            page->second.texture.setSmooth(m_isSmooth);
        }
    }
}

////////////////////////////////////////////////////////////
bool Font::isSmooth() const
{
    return m_isSmooth;
}


////////////////////////////////////////////////////////////
Font& Font::operator =(const Font& right)
{
    Font temp(right);

    std::swap(m_library,     temp.m_library);
    std::swap(m_face,        temp.m_face);
    std::swap(m_streamRec,   temp.m_streamRec);
    std::swap(m_stroker,     temp.m_stroker);
    std::swap(m_refCount,    temp.m_refCount);
    std::swap(m_isSmooth,    temp.m_isSmooth);
    std::swap(m_info,        temp.m_info);
    std::swap(m_pages,       temp.m_pages);
    std::swap(m_pixelBuffer, temp.m_pixelBuffer);

    #ifdef SFML_SYSTEM_ANDROID
        std::swap(m_stream, temp.m_stream);
    #endif

    return *this;
}


////////////////////////////////////////////////////////////
void Font::cleanup()
{
    // Check if we must destroy the FreeType pointers
    if (m_refCount)
    {
        // Decrease the reference counter
        (*m_refCount)--;

        // Free the resources only if we are the last owner
        if (*m_refCount == 0)
        {
            // Delete the reference counter
            delete m_refCount;

            // Destroy the stroker
            if (m_stroker)
                FT_Stroker_Done(static_cast<FT_Stroker>(m_stroker));

            // Destroy the font face
            if (m_face)
                FT_Done_Face(static_cast<FT_Face>(m_face));

            // Destroy the stream rec instance, if any (must be done after FT_Done_Face!)
            if (m_streamRec)
                delete static_cast<FT_StreamRec*>(m_streamRec);

            // Close the library
            if (m_library)
                FT_Done_FreeType(static_cast<FT_Library>(m_library));
        }
    }

    // Reset members
    m_library   = NULL;
    m_face      = NULL;
    m_stroker   = NULL;
    m_streamRec = NULL;
    m_refCount  = NULL;
    m_pages.clear();
    std::vector<Uint8>().swap(m_pixelBuffer);
}


////////////////////////////////////////////////////////////
Font::Page& Font::loadPage(unsigned int characterSize) const
{
    // TODO: Remove this method and use try_emplace instead when updating to C++17
    PageTable::iterator pageIterator = m_pages.find(characterSize);
    if (pageIterator == m_pages.end())
        pageIterator = m_pages.insert(std::make_pair(characterSize, Page(m_isSmooth))).first;

    return pageIterator->second;
}


////////////////////////////////////////////////////////////
Glyph Font::loadGlyph(Uint32 codePoint, unsigned int characterSize, bool bold, float outlineThickness) const
{
    // The glyph to return
    Glyph glyph;

    // First, transform our ugly void* to a FT_Face
    FT_Face face = static_cast<FT_Face>(m_face);
    if (!face)
        return glyph;

    // Set the character size
    if (!setCurrentSize(characterSize))
        return glyph;

    // Load the glyph corresponding to the code point
    FT_Int32 flags = FT_LOAD_TARGET_NORMAL | FT_LOAD_FORCE_AUTOHINT;
    if (outlineThickness != 0)
        flags |= FT_LOAD_NO_BITMAP;
    if (FT_Load_Char(face, codePoint, flags) != 0)
        return glyph;

    // Retrieve the glyph
    FT_Glyph glyphDesc;
    if (FT_Get_Glyph(face->glyph, &glyphDesc) != 0)
        return glyph;

    // Apply bold and outline (there is no fallback for outline) if necessary -- first technique using outline (highest quality)
    FT_Pos weight = 1 << 6;
    bool outline = (glyphDesc->format == FT_GLYPH_FORMAT_OUTLINE);
    if (outline)
    {
        if (bold)
        {
            FT_OutlineGlyph outlineGlyph = reinterpret_cast<FT_OutlineGlyph>(glyphDesc);
            FT_Outline_Embolden(&outlineGlyph->outline, weight);
        }

        if (outlineThickness != 0)
        {
            FT_Stroker stroker = static_cast<FT_Stroker>(m_stroker);

            FT_Stroker_Set(stroker, static_cast<FT_Fixed>(outlineThickness * static_cast<float>(1 << 6)), FT_STROKER_LINECAP_ROUND, FT_STROKER_LINEJOIN_ROUND, 0);
            FT_Glyph_Stroke(&glyphDesc, stroker, true);
        }
    }

    // Convert the glyph to a bitmap (i.e. rasterize it)
    // Warning! After this line, do not read any data from glyphDesc directly, use
    // bitmapGlyph.root to access the FT_Glyph data.
    FT_Glyph_To_Bitmap(&glyphDesc, FT_RENDER_MODE_NORMAL, 0, 1);
    FT_BitmapGlyph bitmapGlyph = reinterpret_cast<FT_BitmapGlyph>(glyphDesc);
    FT_Bitmap& bitmap = bitmapGlyph->bitmap;

    // Apply bold if necessary -- fallback technique using bitmap (lower quality)
    if (!outline)
    {
        if (bold)
            FT_Bitmap_Embolden(static_cast<FT_Library>(m_library), &bitmap, weight, weight);

        if (outlineThickness != 0)
            err() << "Failed to outline glyph (no fallback available)" << std::endl;
    }

    // Compute the glyph's advance offset
    glyph.advance = static_cast<float>(bitmapGlyph->root.advance.x >> 16);
    if (bold)
        glyph.advance += static_cast<float>(weight) / static_cast<float>(1 << 6);

    glyph.lsbDelta = static_cast<int>(face->glyph->lsb_delta);
    glyph.rsbDelta = static_cast<int>(face->glyph->rsb_delta);

    unsigned int width  = bitmap.width;
    unsigned int height = bitmap.rows;

    if ((width > 0) && (height > 0))
    {
        // Leave a small padding around characters, so that filtering doesn't
        // pollute them with pixels from neighbors
        const unsigned int padding = 2;

        width += 2 * padding;
        height += 2 * padding;

        // Get the glyphs page corresponding to the character size
        Page& page = loadPage(characterSize);

        // Find a good position for the new glyph into the texture
        glyph.textureRect = findGlyphRect(page, width, height);

        // Make sure the texture data is positioned in the center
        // of the allocated texture rectangle
        glyph.textureRect.left   += static_cast<int>(padding);
        glyph.textureRect.top    += static_cast<int>(padding);
        glyph.textureRect.width  -= static_cast<int>(2 * padding);
        glyph.textureRect.height -= static_cast<int>(2 * padding);

        // Compute the glyph's bounding box
        glyph.bounds.left   = static_cast<float>( bitmapGlyph->left);
        glyph.bounds.top    = static_cast<float>(-bitmapGlyph->top);
        glyph.bounds.width  = static_cast<float>( bitmap.width);
        glyph.bounds.height = static_cast<float>( bitmap.rows);

        // Resize the pixel buffer to the new size and fill it with transparent white pixels
        m_pixelBuffer.resize(width * height * 4);

        Uint8* current = &m_pixelBuffer[0];
        Uint8* end = current + width * height * 4;

        while (current != end)
        {
            (*current++) = 255;
            (*current++) = 255;
            (*current++) = 255;
            (*current++) = 0;
        }

        // Extract the glyph's pixels from the bitmap
        const Uint8* pixels = bitmap.buffer;
        if (bitmap.pixel_mode == FT_PIXEL_MODE_MONO)
        {
            // Pixels are 1 bit monochrome values
            for (unsigned int y = padding; y < height - padding; ++y)
            {
                for (unsigned int x = padding; x < width - padding; ++x)
                {
                    // The color channels remain white, just fill the alpha channel
                    std::size_t index = x + y * width;
                    m_pixelBuffer[index * 4 + 3] = ((pixels[(x - padding) / 8]) & (1 << (7 - ((x - padding) % 8)))) ? 255 : 0;
                }
                pixels += bitmap.pitch;
            }
        }
        else
        {
            // Pixels are 8 bits gray levels
            for (unsigned int y = padding; y < height - padding; ++y)
            {
                for (unsigned int x = padding; x < width - padding; ++x)
                {
                    // The color channels remain white, just fill the alpha channel
                    std::size_t index = x + y * width;
                    m_pixelBuffer[index * 4 + 3] = pixels[x - padding];
                }
                pixels += bitmap.pitch;
            }
        }

        // Write the pixels to the texture
        unsigned int x = static_cast<unsigned int>(glyph.textureRect.left) - padding;
        unsigned int y = static_cast<unsigned int>(glyph.textureRect.top) - padding;
        unsigned int w = static_cast<unsigned int>(glyph.textureRect.width) + 2 * padding;
        unsigned int h = static_cast<unsigned int>(glyph.textureRect.height) + 2 * padding;
        page.texture.update(&m_pixelBuffer[0], w, h, x, y);
    }

    // Delete the FT glyph
    FT_Done_Glyph(glyphDesc);

    // Done :)
    return glyph;
}


////////////////////////////////////////////////////////////
IntRect Font::findGlyphRect(Page& page, unsigned int width, unsigned int height) const
{
    // Find the line that fits well the glyph
    Row* row = NULL;
    float bestRatio = 0;
    for (std::vector<Row>::iterator it = page.rows.begin(); it != page.rows.end() && !row; ++it)
    {
        float ratio = static_cast<float>(height) / static_cast<float>(it->height);

        // Ignore rows that are either too small or too high
        if ((ratio < 0.7f) || (ratio > 1.f))
            continue;

        // Check if there's enough horizontal space left in the row
        if (width > page.texture.getSize().x - it->width)
            continue;

        // Make sure that this new row is the best found so far
        if (ratio < bestRatio)
            continue;

        // The current row passed all the tests: we can select it
        row = &*it;
        bestRatio = ratio;
    }

    // If we didn't find a matching row, create a new one (10% taller than the glyph)
    if (!row)
    {
        unsigned int rowHeight = height + height / 10;
        while ((page.nextRow + rowHeight >= page.texture.getSize().y) || (width >= page.texture.getSize().x))
        {
            // Not enough space: resize the texture if possible
            unsigned int textureWidth  = page.texture.getSize().x;
            unsigned int textureHeight = page.texture.getSize().y;
            if ((textureWidth * 2 <= Texture::getMaximumSize()) && (textureHeight * 2 <= Texture::getMaximumSize()))
            {
                // Make the texture 2 times bigger
                Texture newTexture;
                newTexture.create(textureWidth * 2, textureHeight * 2);
                newTexture.setSmooth(m_isSmooth);
                newTexture.update(page.texture);
                page.texture.swap(newTexture);
            }
            else
            {
                // Oops, we've reached the maximum texture size...
                err() << "Failed to add a new character to the font: the maximum texture size has been reached" << std::endl;
                return IntRect(0, 0, 2, 2);
            }
        }

        // We can now create the new row
        page.rows.push_back(Row(page.nextRow, rowHeight));
        page.nextRow += rowHeight;
        row = &page.rows.back();
    }

    // Find the glyph's rectangle on the selected row
    IntRect rect(Rect<unsigned int>(row->width, row->top, width, height));

    // Update the row informations
    row->width += width;

    return rect;
}


////////////////////////////////////////////////////////////
bool Font::setCurrentSize(unsigned int characterSize) const
{
    // FT_Set_Pixel_Sizes is an expensive function, so we must call it
    // only when necessary to avoid killing performances

    FT_Face face = static_cast<FT_Face>(m_face);
    FT_UShort currentSize = face->size->metrics.x_ppem;

    if (currentSize != characterSize)
    {
        FT_Error result = FT_Set_Pixel_Sizes(face, 0, characterSize);

        if (result == FT_Err_Invalid_Pixel_Size)
        {
            // In the case of bitmap fonts, resizing can
            // fail if the requested size is not available
            if (!FT_IS_SCALABLE(face))
            {
                err() << "Failed to set bitmap font size to " << characterSize << std::endl;
                err() << "Available sizes are: ";
                for (int i = 0; i < face->num_fixed_sizes; ++i)
                {
                    const long size = (face->available_sizes[i].y_ppem + 32) >> 6;
                    err() << size << " ";
                }
                err() << std::endl;
            }
            else
            {
                err() << "Failed to set font size to " << characterSize << std::endl;
            }
        }

        return result == FT_Err_Ok;
    }

     return true;
}


////////////////////////////////////////////////////////////
Font::Page::Page(bool smooth) :
nextRow(3)
{
    // Make sure that the texture is initialized by default
    sf::Image image;
    image.create(128, 128, Color(255, 255, 255, 0));

    // Reserve a 2x2 white square for texturing underlines
    for (unsigned int x = 0; x < 2; ++x)
        for (unsigned int y = 0; y < 2; ++y)
            image.setPixel(x, y, Color(255, 255, 255, 255));

    // Create the texture
    texture.loadFromImage(image);
    texture.setSmooth(smooth);
}

} // namespace sf


// == Rust bindings ==

struct sfGlyph {
    float advance;         ///< Offset to move horizontically to the next character
    sfFloatRect bounds;    ///< Bounding rectangle of the glyph, in coordinates relative to the baseline
    sfIntRect textureRect; ///< Texture coordinates of the glyph inside the font's image
};

extern "C" sf::Font *sfFont_new() {
    return new sf::Font;
}

extern "C" void sfFont_del(sf::Font *font) {
    delete font;
}

extern "C" sf::Font *sfFont_cpy(const sf::Font *font) {
    return new sf::Font(*font);
}

extern "C" bool sfFont_loadFromFile(sf::Font *font, const char *filename) {
    return font->loadFromFile(filename);
}

extern "C" bool sfFont_loadFromMemory(sf::Font *font, const uint8_t *data, size_t sizeInBytes) {
    return font->loadFromMemory(data, sizeInBytes);
}

extern "C" sfGlyph sfFont_getGlyph(const sf::Font *font, uint32_t codePoint, unsigned int characterSize, bool bold, float outlineThickness) {
    sf::Glyph glyph = font->getGlyph(codePoint, characterSize, bold, outlineThickness);
    return {
        glyph.advance,
        {glyph.bounds.left, glyph.bounds.top, glyph.bounds.width, glyph.bounds.height},
        {glyph.textureRect.left, glyph.textureRect.top, glyph.textureRect.width, glyph.textureRect.height}};
}

extern "C" float sfFont_getKerning(const sf::Font *font, uint32_t first, uint32_t second, unsigned int characterSize) {
    return font->getKerning(first, second, characterSize);
}

extern "C" float sfFont_getBoldKerning(const sf::Font *font, uint32_t first, uint32_t second, unsigned int characterSize) {
    return font->getKerning(first, second, characterSize, true);
}

extern "C" float sfFont_getLineSpacing(const sf::Font *font, unsigned int characterSize) {
    return font->getLineSpacing(characterSize);
}

extern "C" float sfFont_getUnderlinePosition(const sf::Font *font, unsigned int characterSize) {
    return font->getUnderlinePosition(characterSize);
}

extern "C" float sfFont_getUnderlineThickness(const sf::Font *font, unsigned int characterSize) {
    return font->getUnderlineThickness(characterSize);
}

extern "C" const sf::Texture *sfFont_getTexture(const sf::Font *font, unsigned int characterSize) {
    return &font->getTexture(characterSize);
}

extern "C" bool sfFont_isSmooth(const sf::Font *font) {
    return font->isSmooth();
}

extern "C" void sfFont_setSmooth(sf::Font *font, bool smooth) {
    font->setSmooth(smooth);
}

struct sfFontInfo {
    const char *family;
};

extern "C" sfFontInfo sfFont_getInfo(const sf::Font *font) {
    return {font->getInfo().family.c_str()};
}
